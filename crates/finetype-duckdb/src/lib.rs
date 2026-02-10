//! FineType DuckDB Extension
//!
//! Provides scalar functions for semantic type classification:
//! - `finetype_version()` — Returns the extension version
//! - `finetype(value)` — Classify a single value (planned)
//! - `finetype_profile(col)` — Profile a column with distribution analysis (planned)

use duckdb::core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId};
use duckdb::vscalar::{ScalarFunctionSignature, VScalar};
use duckdb::vtab::arrow::WritableVector;
use duckdb::{duckdb_entrypoint_c_api, Result};
use std::error::Error;
use std::ffi::CString;

// ═══════════════════════════════════════════════════════════════════════════════
// EMBEDDED RESOURCES
// ═══════════════════════════════════════════════════════════════════════════════

/// Embed the tier_graph.json metadata at compile time.
/// The actual model weights (.safetensors) will be embedded when models are finalized.
#[cfg(feature = "embed-models")]
const _TIER_GRAPH_JSON: &[u8] = include_bytes!("../../../models/tiered/tier_graph.json");

/// Extension name and version.
const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

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

    Ok(())
}
