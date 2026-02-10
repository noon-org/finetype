//! FineType DuckDB Extension
//!
//! Provides scalar functions for semantic type classification:
//! - `finetype_version()` — Returns the extension version
//! - `finetype(value)` — Classify a single value, returns the semantic type label
//! - `finetype_detail(value)` — Classify with detail: returns JSON with type, confidence, DuckDB type
//! - `finetype_cast(value)` — Normalize a value for safe TRY_CAST (dates → ISO, booleans → true/false, etc.)
//! - `finetype_unpack(json)` — Recursively classify JSON fields, returns annotated JSON

use duckdb::core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId};
use duckdb::vscalar::{ScalarFunctionSignature, VScalar};
use duckdb::vtab::arrow::WritableVector;
use duckdb::{duckdb_entrypoint_c_api, Result};
use std::error::Error;
use std::ffi::CString;

mod type_mapping;

#[cfg(feature = "embed-models")]
mod normalize;
#[cfg(feature = "embed-models")]
mod unpack;

// ═══════════════════════════════════════════════════════════════════════════════
// EMBEDDED MODELS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "embed-models")]
mod embedded {
    include!(concat!(env!("OUT_DIR"), "/embedded_models.rs"));
}

/// Extension name and version.
const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

// ═══════════════════════════════════════════════════════════════════════════════
// GLOBAL CLASSIFIER (lazy-initialized on first use)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "embed-models")]
use std::sync::OnceLock;

/// Global flat classifier, initialized on first finetype() call.
/// Uses the single-pass CharCNN model (91.97% accuracy, ~100x faster than tiered).
#[cfg(feature = "embed-models")]
static CLASSIFIER: OnceLock<finetype_model::CharClassifier> = OnceLock::new();

/// Initialize or get the global classifier from embedded flat model.
#[cfg(feature = "embed-models")]
fn get_classifier() -> &'static finetype_model::CharClassifier {
    CLASSIFIER.get_or_init(|| {
        finetype_model::CharClassifier::from_bytes(
            embedded::FLAT_WEIGHTS,
            embedded::FLAT_LABELS,
            embedded::FLAT_CONFIG,
        )
        .expect("Failed to load embedded flat model")
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// VARCHAR HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Read a VARCHAR value from a DuckDB data chunk at a specific column and row.
///
/// Returns None if the value is NULL.
#[cfg(feature = "embed-models")]
unsafe fn read_varchar(
    input: &mut DataChunkHandle,
    col_idx: usize,
    row_idx: usize,
) -> Option<String> {
    use libduckdb_sys::*;

    let raw_chunk = input.get_ptr();
    let vector = duckdb_data_chunk_get_vector(raw_chunk, col_idx as idx_t);

    // Check validity (NULL check)
    let validity = duckdb_vector_get_validity(vector);
    if !validity.is_null() {
        let entry = row_idx / 64;
        let bit = row_idx % 64;
        let mask = *validity.add(entry);
        if (mask >> bit) & 1 == 0 {
            return None;
        }
    }

    // Read string data
    let data = duckdb_vector_get_data(vector) as *const duckdb_string_t;
    let str_val = *data.add(row_idx);

    let (ptr, len) = if duckdb_string_is_inlined(str_val) {
        (
            str_val.value.inlined.inlined.as_ptr() as *const u8,
            str_val.value.inlined.length as usize,
        )
    } else {
        (
            str_val.value.pointer.ptr as *const u8,
            str_val.value.pointer.length as usize,
        )
    };

    if ptr.is_null() || len == 0 {
        return Some(String::new());
    }

    let bytes = std::slice::from_raw_parts(ptr, len);
    std::str::from_utf8(bytes).ok().map(|s| s.to_string())
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCALAR FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// `finetype_version()` — Returns the FineType extension version string.
struct FineTypeVersion;

impl VScalar for FineTypeVersion {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let len = input.len();
        let output_vec = output.flat_vector();
        let version = CString::new(format!("finetype {}", EXTENSION_VERSION))?;
        for i in 0..len {
            output_vec.insert(i, version.clone());
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![],
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
        )]
    }
}

/// `finetype(value VARCHAR) → VARCHAR` — Classify a single value.
///
/// Returns the full semantic type label (e.g. "technology.internet.url",
/// "datetime.date.iso", "identity.person.email").
#[cfg(feature = "embed-models")]
struct FineType;

#[cfg(feature = "embed-models")]
impl VScalar for FineType {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let classifier = get_classifier();
        let len = input.len();
        let output_vec = output.flat_vector();

        // Collect non-null values for batch inference
        let mut indices: Vec<usize> = Vec::with_capacity(len);
        let mut texts: Vec<String> = Vec::with_capacity(len);

        for i in 0..len {
            if let Some(text) = read_varchar(input, 0, i) {
                if !text.is_empty() {
                    indices.push(i);
                    texts.push(text);
                } else {
                    // Empty string → unknown
                    let cstr = CString::new("unknown")?;
                    output_vec.insert(i, cstr);
                }
            }
            // NULL values: DuckDB handles NULL propagation for scalar functions
        }

        // Batch classify all non-null, non-empty values
        if !texts.is_empty() {
            let results = classifier.classify_batch(&texts)?;
            for (idx, result) in indices.iter().zip(results.iter()) {
                let label = CString::new(result.label.as_str())?;
                output_vec.insert(*idx, label);
            }
        }

        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeHandle::from(LogicalTypeId::Varchar)],
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
        )]
    }
}

/// `finetype_detail(value VARCHAR) → VARCHAR` — Classify with full detail.
///
/// Returns a JSON object with:
/// - `type`: semantic type label
/// - `confidence`: model confidence (0.0 to 1.0)
/// - `duckdb_type`: recommended DuckDB CAST target type
#[cfg(feature = "embed-models")]
struct FineTypeDetail;

#[cfg(feature = "embed-models")]
impl VScalar for FineTypeDetail {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let classifier = get_classifier();
        let len = input.len();
        let output_vec = output.flat_vector();

        for i in 0..len {
            if let Some(text) = read_varchar(input, 0, i) {
                if text.is_empty() {
                    let cstr = CString::new(
                        r#"{"type":"unknown","confidence":0.0,"duckdb_type":"VARCHAR"}"#,
                    )?;
                    output_vec.insert(i, cstr);
                    continue;
                }
                let result = classifier.classify(&text)?;
                let duckdb_type = type_mapping::to_duckdb_type(&result.label);
                let json = format!(
                    r#"{{"type":"{}","confidence":{:.4},"duckdb_type":"{}"}}"#,
                    result.label, result.confidence, duckdb_type
                );
                let cstr = CString::new(json)?;
                output_vec.insert(i, cstr);
            }
        }

        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeHandle::from(LogicalTypeId::Varchar)],
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
        )]
    }
}

/// `finetype_cast(value VARCHAR) → VARCHAR` — Normalize a value for safe casting.
///
/// Classifies the value, then normalizes it to a canonical form suitable for
/// DuckDB `TRY_CAST()`. Returns NULL if the value doesn't validate for its
/// detected type.
///
/// Examples:
/// - `finetype_cast('01/15/2024')` → `'2024-01-15'` (US date → ISO)
/// - `finetype_cast('Yes')` → `'true'` (boolean normalization)
/// - `finetype_cast('550E8400-...')` → `'550e8400-...'` (UUID lowercase)
/// - `finetype_cast('1,234')` → `'1234'` (strip formatting)
#[cfg(feature = "embed-models")]
struct FineTypeCast;

#[cfg(feature = "embed-models")]
impl VScalar for FineTypeCast {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let classifier = get_classifier();
        let len = input.len();
        let mut output_vec = output.flat_vector();

        for i in 0..len {
            if let Some(text) = read_varchar(input, 0, i) {
                if text.is_empty() {
                    output_vec.set_null(i);
                    continue;
                }
                match classifier.classify(&text) {
                    Ok(result) => {
                        if let Some(normalized) = normalize::normalize(&text, &result.label) {
                            let cstr = CString::new(normalized)?;
                            output_vec.insert(i, cstr);
                        } else {
                            // Validation failed → NULL
                            output_vec.set_null(i);
                        }
                    }
                    Err(_) => {
                        // Classification error → pass through
                        let cstr = CString::new(text)?;
                        output_vec.insert(i, cstr);
                    }
                }
            }
            // NULL input → DuckDB handles NULL propagation
        }

        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeHandle::from(LogicalTypeId::Varchar)],
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
        )]
    }
}

/// `finetype_unpack(json_value VARCHAR) → VARCHAR` — Recursively infer types in JSON.
///
/// Parses a JSON string and classifies each scalar value. Returns annotated JSON
/// where each value is replaced with an object containing:
/// - `value`: the original value
/// - `type`: detected finetype label
/// - `confidence`: classification confidence (0.0 to 1.0)
/// - `duckdb_type`: recommended DuckDB type
///
/// Returns NULL for non-JSON input.
#[cfg(feature = "embed-models")]
struct FineTypeUnpack;

#[cfg(feature = "embed-models")]
impl VScalar for FineTypeUnpack {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let classifier = get_classifier();
        let len = input.len();
        let mut output_vec = output.flat_vector();

        for i in 0..len {
            if let Some(text) = read_varchar(input, 0, i) {
                if text.is_empty() {
                    output_vec.set_null(i);
                    continue;
                }
                match unpack::unpack_json(&text, classifier) {
                    Some(annotated) => {
                        let cstr = CString::new(annotated)?;
                        output_vec.insert(i, cstr);
                    }
                    None => {
                        // Not valid JSON → NULL
                        output_vec.set_null(i);
                    }
                }
            }
        }

        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeHandle::from(LogicalTypeId::Varchar)],
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
        )]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXTENSION ENTRYPOINT
// ═══════════════════════════════════════════════════════════════════════════════

/// # Safety
///
/// Called by DuckDB when loading the extension. The Connection is valid for the
/// lifetime of the extension.
#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: duckdb::Connection) -> Result<(), Box<dyn Error>> {
    con.register_scalar_function::<FineTypeVersion>("finetype_version")
        .expect("Failed to register finetype_version");

    #[cfg(feature = "embed-models")]
    {
        con.register_scalar_function::<FineType>("finetype")
            .expect("Failed to register finetype");

        con.register_scalar_function::<FineTypeDetail>("finetype_detail")
            .expect("Failed to register finetype_detail");

        con.register_scalar_function::<FineTypeCast>("finetype_cast")
            .expect("Failed to register finetype_cast");

        con.register_scalar_function::<FineTypeUnpack>("finetype_unpack")
            .expect("Failed to register finetype_unpack");
    }

    Ok(())
}
