//! Synthetic data generation for training.
//!
//! Replaces the Python/mimesis data generation with pure Rust.

use crate::taxonomy::{Definition, Taxonomy};
use chrono::{NaiveDate, NaiveDateTime};
use rand::prelude::*;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unknown label: {0}")]
    UnknownLabel(String),
    #[error("Generator not implemented for: {0}")]
    NotImplemented(String),
}

/// A generated sample with its label.
#[derive(Debug, Clone)]
pub struct Sample {
    pub text: String,
    pub label: String,
}

/// Data generator for creating synthetic training samples.
pub struct Generator {
    taxonomy: Taxonomy,
    rng: StdRng,
}

impl Generator {
    /// Create a new generator with the given taxonomy.
    pub fn new(taxonomy: Taxonomy) -> Self {
        Self {
            taxonomy,
            rng: StdRng::from_entropy(),
        }
    }

    /// Create a generator with a fixed seed for reproducibility.
    pub fn with_seed(taxonomy: Taxonomy, seed: u64) -> Self {
        Self {
            taxonomy,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Generate samples for all labels at a given priority level.
    pub fn generate_all(&mut self, min_priority: u8, samples_per_label: usize) -> Vec<Sample> {
        // Collect definitions first to avoid borrow issues
        let definitions: Vec<_> = self.taxonomy
            .at_priority(min_priority)
            .into_iter()
            .map(|d| (d.provider.clone(), d.method.clone()))
            .collect();
        
        let mut all_samples = Vec::new();

        for (provider, method) in definitions {
            let label = format!("{}.{}", provider, method);
            for _ in 0..samples_per_label {
                if let Ok(text) = self.generate_value(&provider, &method) {
                    all_samples.push(Sample { text, label: label.clone() });
                }
            }
        }

        all_samples
    }

    /// Generate samples for a specific definition.
    pub fn generate_for_definition(
        &mut self,
        def: &Definition,
        count: usize,
    ) -> Result<Vec<Sample>, GeneratorError> {
        let label = def.label();
        let mut samples = Vec::with_capacity(count);

        for _ in 0..count {
            let text = self.generate_value(&def.provider, &def.method)?;
            samples.push(Sample {
                text,
                label: label.clone(),
            });
        }

        Ok(samples)
    }

    /// Generate a single value for a provider.method combination.
    fn generate_value(&mut self, provider: &str, method: &str) -> Result<String, GeneratorError> {
        match (provider, method) {
            // Datetime
            ("datetime", "iso_8601") => Ok(self.gen_iso_8601()),
            ("datetime", "iso_8601_compact") => Ok(self.gen_iso_8601_compact()),
            ("datetime", "iso_8601_ext") => Ok(self.gen_iso_8601_ext()),
            ("datetime", "rfc_2822") => Ok(self.gen_rfc_2822()),
            ("datetime", "rfc_3339") => Ok(self.gen_rfc_3339()),
            ("datetime", "unix_timestamp") => Ok(self.gen_unix_timestamp()),
            ("datetime", "timestamp") => Ok(self.gen_unix_timestamp()),
            ("datetime", "sql_standard") => Ok(self.gen_sql_datetime()),
            ("datetime", "american") => Ok(self.gen_american_datetime()),
            ("datetime", "european") => Ok(self.gen_european_datetime()),
            ("datetime", "date") => Ok(self.gen_date()),
            ("datetime", "datetime") => Ok(self.gen_datetime()),
            ("datetime", "time") => Ok(self.gen_time()),
            ("datetime", "timezone") => Ok(self.gen_timezone()),
            ("datetime", "gmt_offset") => Ok(self.gen_gmt_offset()),
            ("date", "abbreviated_month") => Ok(self.gen_abbreviated_month()),
            ("date", "julian") => Ok(self.gen_julian_date()),
            ("date", "ordinal") => Ok(self.gen_ordinal_date()),

            // Internet
            ("internet", "ip_v4") => Ok(self.gen_ipv4()),
            ("internet", "ip_v4_with_port") => Ok(self.gen_ipv4_with_port()),
            ("internet", "ip_v6") => Ok(self.gen_ipv6()),
            ("internet", "mac_address") => Ok(self.gen_mac_address()),
            ("internet", "url") => Ok(self.gen_url()),
            ("internet", "slug") => Ok(self.gen_slug()),
            ("internet", "query_string") => Ok(self.gen_query_string()),
            ("internet", "user_agent") => Ok(self.gen_user_agent()),
            ("internet", "top_level_domain") => Ok(self.gen_tld()),
            ("internet", "public_dns") => Ok(self.gen_public_dns()),
            ("internet", "asn") => Ok(self.gen_asn()),

            // Cryptographic
            ("cryptographic", "uuid") => Ok(self.gen_uuid()),
            ("cryptographic", "hash") => Ok(self.gen_hash()),
            ("cryptographic", "token_hex") => Ok(self.gen_token_hex()),
            ("cryptographic", "token_urlsafe") => Ok(self.gen_token_urlsafe()),

            // Person
            ("person", "email") => Ok(self.gen_email()),
            ("person", "first_name") => Ok(self.gen_first_name()),
            ("person", "last_name") => Ok(self.gen_last_name()),
            ("person", "full_name") => Ok(self.gen_full_name()),
            ("person", "phone_number") => Ok(self.gen_phone_number()),

            // Address
            ("address", "city") => Ok(self.gen_city()),
            ("address", "country") => Ok(self.gen_country()),
            ("address", "country_code") => Ok(self.gen_country_code()),
            ("address", "postal_code") => Ok(self.gen_postal_code()),
            ("address", "iata_code") => Ok(self.gen_iata_code()),
            ("address", "icao_code") => Ok(self.gen_icao_code()),

            // Code
            ("code", "ean") => Ok(self.gen_ean()),
            ("code", "imei") => Ok(self.gen_imei()),
            ("code", "isbn") => Ok(self.gen_isbn()),
            ("code", "issn") => Ok(self.gen_issn()),
            ("code", "locale_code") => Ok(self.gen_locale_code()),

            // Payment
            ("payment", "bitcoin_address") => Ok(self.gen_bitcoin_address()),
            ("payment", "ethereum_address") => Ok(self.gen_ethereum_address()),
            ("payment", "credit_card_number") => Ok(self.gen_credit_card()),

            // Finance
            ("finance", "currency_iso_code") => Ok(self.gen_currency_code()),
            ("finance", "cryptocurrency_iso_code") => Ok(self.gen_crypto_code()),
            ("finance", "stock_ticker") => Ok(self.gen_stock_ticker()),

            // Science
            ("science", "dna_sequence") => Ok(self.gen_dna_sequence()),
            ("science", "rna_sequence") => Ok(self.gen_rna_sequence()),

            // Development
            ("development", "version") => Ok(self.gen_semver()),
            ("development", "boolean") => Ok(self.gen_boolean()),

            // File
            ("file", "extension") => Ok(self.gen_file_extension()),
            ("file", "file_name") => Ok(self.gen_file_name()),
            ("file", "mime_type") => Ok(self.gen_mime_type()),

            _ => Err(GeneratorError::NotImplemented(format!(
                "{}.{}",
                provider, method
            ))),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Datetime generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_iso_8601(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    fn gen_iso_8601_compact(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%Y%m%dT%H%M%S").to_string()
    }

    fn gen_iso_8601_ext(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{}Z", dt.format("%Y-%m-%dT%H:%M:%S%.6f"))
    }

    fn gen_rfc_2822(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%a, %d %b %Y %H:%M:%S GMT+00:00").to_string()
    }

    fn gen_rfc_3339(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{} GMT+00:00", dt.format("%Y-%m-%dT%H:%M:%S"))
    }

    fn gen_unix_timestamp(&mut self) -> String {
        let ts = self.rng.gen_range(1_000_000_000i64..2_000_000_000);
        ts.to_string()
    }

    fn gen_sql_datetime(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn gen_american_datetime(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%m/%d/%Y %I:%M %p").to_string()
    }

    fn gen_european_datetime(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%d/%m/%Y %H:%M").to_string()
    }

    fn gen_date(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%Y-%m-%d").to_string()
    }

    fn gen_datetime(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{}", dt.format("%Y-%m-%dT%H:%M:%S%.6f"))
    }

    fn gen_time(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{}", dt.format("%H:%M:%S%.6f"))
    }

    fn gen_timezone(&mut self) -> String {
        let timezones = [
            "America/New_York",
            "Europe/London",
            "Asia/Tokyo",
            "Australia/Sydney",
            "Pacific/Auckland",
            "America/Los_Angeles",
            "Europe/Paris",
            "Asia/Shanghai",
        ];
        timezones[self.rng.gen_range(0..timezones.len())].to_string()
    }

    fn gen_gmt_offset(&mut self) -> String {
        let offset = self.rng.gen_range(-12..=14);
        let sign = if offset >= 0 { "+" } else { "" };
        format!("UTC {}{}:00", sign, offset)
    }

    fn gen_abbreviated_month(&mut self) -> String {
        let dt = self.random_datetime();
        dt.format("%b %d, %Y").to_string()
    }

    fn gen_julian_date(&mut self) -> String {
        let year = self.rng.gen_range(20..30);
        let day = self.rng.gen_range(1..366);
        format!("{:02}-{:03}", year, day)
    }

    fn gen_ordinal_date(&mut self) -> String {
        let year = self.rng.gen_range(2020..2030);
        let day = self.rng.gen_range(1..366);
        format!("{}-{:03}", year, day)
    }

    fn random_datetime(&mut self) -> NaiveDateTime {
        let year = self.rng.gen_range(2020..2030);
        let month = self.rng.gen_range(1..=12);
        let day = self.rng.gen_range(1..=28);
        let hour = self.rng.gen_range(0..24);
        let minute = self.rng.gen_range(0..60);
        let second = self.rng.gen_range(0..60);
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Internet generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_ipv4(&mut self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.rng.gen_range(1..255),
            self.rng.gen_range(0..255),
            self.rng.gen_range(0..255),
            self.rng.gen_range(1..255)
        )
    }

    fn gen_ipv4_with_port(&mut self) -> String {
        format!("{}:{}", self.gen_ipv4(), self.rng.gen_range(1024..65535))
    }

    fn gen_ipv6(&mut self) -> String {
        let segments: Vec<String> = (0..8)
            .map(|_| format!("{:04x}", self.rng.gen::<u16>()))
            .collect();
        segments.join(":")
    }

    fn gen_mac_address(&mut self) -> String {
        let octets: Vec<String> = (0..6)
            .map(|_| format!("{:02x}", self.rng.gen::<u8>()))
            .collect();
        octets.join(":")
    }

    fn gen_url(&mut self) -> String {
        let domains = ["example", "test", "demo", "sample", "mysite"];
        let tlds = ["com", "org", "net", "io", "dev"];
        format!(
            "https://{}.{}/",
            domains[self.rng.gen_range(0..domains.len())],
            tlds[self.rng.gen_range(0..tlds.len())]
        )
    }

    fn gen_slug(&mut self) -> String {
        let words = ["hello", "world", "test", "demo", "sample", "example"];
        let count = self.rng.gen_range(2..5);
        let selected: Vec<&str> = (0..count)
            .map(|_| words[self.rng.gen_range(0..words.len())])
            .collect();
        selected.join("-")
    }

    fn gen_query_string(&mut self) -> String {
        let keys = ["id", "page", "sort", "filter", "q"];
        let values = ["1", "test", "asc", "true", "search"];
        format!(
            "{}={}",
            keys[self.rng.gen_range(0..keys.len())],
            values[self.rng.gen_range(0..values.len())]
        )
    }

    fn gen_user_agent(&mut self) -> String {
        let agents = [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
        ];
        agents[self.rng.gen_range(0..agents.len())].to_string()
    }

    fn gen_tld(&mut self) -> String {
        let tlds = [".com", ".org", ".net", ".io", ".dev", ".ai", ".co"];
        tlds[self.rng.gen_range(0..tlds.len())].to_string()
    }

    fn gen_public_dns(&mut self) -> String {
        let dns = ["8.8.8.8", "8.8.4.4", "1.1.1.1", "1.0.0.1", "208.67.222.222"];
        dns[self.rng.gen_range(0..dns.len())].to_string()
    }

    fn gen_asn(&mut self) -> String {
        format!("AS{}", self.rng.gen_range(1000..4000000000u64))
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Cryptographic generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_uuid(&mut self) -> String {
        Uuid::new_v4().to_string()
    }

    fn gen_hash(&mut self) -> String {
        let bytes: Vec<String> = (0..32)
            .map(|_| format!("{:02x}", self.rng.gen::<u8>()))
            .collect();
        bytes.join("")
    }

    fn gen_token_hex(&mut self) -> String {
        let bytes: Vec<String> = (0..32)
            .map(|_| format!("{:02x}", self.rng.gen::<u8>()))
            .collect();
        bytes.join("")
    }

    fn gen_token_urlsafe(&mut self) -> String {
        use rand::distributions::Alphanumeric;
        let token: String = (0..43)
            .map(|_| self.rng.sample(Alphanumeric) as char)
            .collect();
        token.replace('+', "-").replace('/', "_")
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Person generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_email(&mut self) -> String {
        let names = ["john", "jane", "bob", "alice", "charlie", "diana"];
        let domains = ["example.com", "test.org", "mail.net", "email.io"];
        format!(
            "{}{}@{}",
            names[self.rng.gen_range(0..names.len())],
            self.rng.gen_range(1..999),
            domains[self.rng.gen_range(0..domains.len())]
        )
    }

    fn gen_first_name(&mut self) -> String {
        let names = [
            "James", "Mary", "John", "Patricia", "Robert", "Jennifer", "Michael", "Linda",
            "William", "Elizabeth",
        ];
        names[self.rng.gen_range(0..names.len())].to_string()
    }

    fn gen_last_name(&mut self) -> String {
        let names = [
            "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
            "Rodriguez", "Martinez",
        ];
        names[self.rng.gen_range(0..names.len())].to_string()
    }

    fn gen_full_name(&mut self) -> String {
        format!("{} {}", self.gen_first_name(), self.gen_last_name())
    }

    fn gen_phone_number(&mut self) -> String {
        format!(
            "+1-{:03}-{:03}-{:04}",
            self.rng.gen_range(200..999),
            self.rng.gen_range(200..999),
            self.rng.gen_range(1000..9999)
        )
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Address generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_city(&mut self) -> String {
        let cities = [
            "New York",
            "Los Angeles",
            "Chicago",
            "Houston",
            "Phoenix",
            "Philadelphia",
            "San Antonio",
            "San Diego",
        ];
        cities[self.rng.gen_range(0..cities.len())].to_string()
    }

    fn gen_country(&mut self) -> String {
        let countries = [
            "United States",
            "Canada",
            "United Kingdom",
            "Germany",
            "France",
            "Australia",
            "Japan",
            "Brazil",
        ];
        countries[self.rng.gen_range(0..countries.len())].to_string()
    }

    fn gen_country_code(&mut self) -> String {
        let codes = ["US", "CA", "GB", "DE", "FR", "AU", "JP", "BR"];
        codes[self.rng.gen_range(0..codes.len())].to_string()
    }

    fn gen_postal_code(&mut self) -> String {
        format!("{:05}", self.rng.gen_range(10000..99999))
    }

    fn gen_iata_code(&mut self) -> String {
        let chars: String = (0..3)
            .map(|_| (b'A' + self.rng.gen_range(0..26)) as char)
            .collect();
        chars
    }

    fn gen_icao_code(&mut self) -> String {
        let chars: String = (0..4)
            .map(|_| (b'A' + self.rng.gen_range(0..26)) as char)
            .collect();
        chars
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Code generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_ean(&mut self) -> String {
        format!("{:013}", self.rng.gen_range(1000000000000u64..9999999999999))
    }

    fn gen_imei(&mut self) -> String {
        format!("{:015}", self.rng.gen_range(100000000000000u64..999999999999999))
    }

    fn gen_isbn(&mut self) -> String {
        format!(
            "{}-{}-{:05}-{:03}-{}",
            self.rng.gen_range(1..9),
            self.rng.gen_range(1..9),
            self.rng.gen_range(10000..99999),
            self.rng.gen_range(100..999),
            self.rng.gen_range(0..9)
        )
    }

    fn gen_issn(&mut self) -> String {
        format!(
            "{:04}-{:04}",
            self.rng.gen_range(1000..9999),
            self.rng.gen_range(1000..9999)
        )
    }

    fn gen_locale_code(&mut self) -> String {
        let locales = ["en-us", "en-gb", "de-de", "fr-fr", "es-es", "ja-jp", "zh-cn"];
        locales[self.rng.gen_range(0..locales.len())].to_string()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Payment generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_bitcoin_address(&mut self) -> String {
        let prefix = ["1", "3", "bc1"][self.rng.gen_range(0..3)];
        let chars: String = (0..33)
            .map(|_| {
                let idx = self.rng.gen_range(0..58);
                "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
                    .chars()
                    .nth(idx)
                    .unwrap()
            })
            .collect();
        format!("{}{}", prefix, chars)
    }

    fn gen_ethereum_address(&mut self) -> String {
        let hex: String = (0..40)
            .map(|_| format!("{:x}", self.rng.gen_range(0..16)))
            .collect();
        format!("0x{}", hex)
    }

    fn gen_credit_card(&mut self) -> String {
        // Generate a Visa-like number
        format!(
            "4{:03} {:04} {:04} {:04}",
            self.rng.gen_range(100..999),
            self.rng.gen_range(1000..9999),
            self.rng.gen_range(1000..9999),
            self.rng.gen_range(1000..9999)
        )
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Finance generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_currency_code(&mut self) -> String {
        let codes = ["USD", "EUR", "GBP", "JPY", "AUD", "CAD", "CHF", "CNY"];
        codes[self.rng.gen_range(0..codes.len())].to_string()
    }

    fn gen_crypto_code(&mut self) -> String {
        let codes = ["BTC", "ETH", "XRP", "LTC", "BCH", "ADA", "DOT", "LINK"];
        codes[self.rng.gen_range(0..codes.len())].to_string()
    }

    fn gen_stock_ticker(&mut self) -> String {
        let tickers = ["AAPL", "GOOGL", "MSFT", "AMZN", "META", "TSLA", "NVDA", "JPM"];
        tickers[self.rng.gen_range(0..tickers.len())].to_string()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Science generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_dna_sequence(&mut self) -> String {
        let bases = ['A', 'C', 'G', 'T'];
        (0..10)
            .map(|_| bases[self.rng.gen_range(0..4)])
            .collect()
    }

    fn gen_rna_sequence(&mut self) -> String {
        let bases = ['A', 'C', 'G', 'U'];
        (0..10)
            .map(|_| bases[self.rng.gen_range(0..4)])
            .collect()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Development generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_semver(&mut self) -> String {
        format!(
            "{}.{}.{}",
            self.rng.gen_range(0..100),
            self.rng.gen_range(0..100),
            self.rng.gen_range(0..100)
        )
    }

    fn gen_boolean(&mut self) -> String {
        if self.rng.gen_bool(0.5) {
            "true"
        } else {
            "false"
        }
        .to_string()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // File generators
    // ─────────────────────────────────────────────────────────────────────────

    fn gen_file_extension(&mut self) -> String {
        let exts = [".txt", ".csv", ".json", ".xml", ".pdf", ".png", ".jpg"];
        exts[self.rng.gen_range(0..exts.len())].to_string()
    }

    fn gen_file_name(&mut self) -> String {
        let names = ["document", "report", "data", "image", "file"];
        let exts = ["txt", "csv", "json", "pdf", "png"];
        format!(
            "{}.{}",
            names[self.rng.gen_range(0..names.len())],
            exts[self.rng.gen_range(0..exts.len())]
        )
    }

    fn gen_mime_type(&mut self) -> String {
        let types = [
            "text/plain",
            "text/html",
            "application/json",
            "application/pdf",
            "image/png",
            "image/jpeg",
        ];
        types[self.rng.gen_range(0..types.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_generation() {
        let taxonomy = Taxonomy::from_yaml("").unwrap_or_else(|_| {
            // Empty taxonomy for testing
            Taxonomy::from_yaml("internet.ip_v4:\n  provider: internet\n  method: ip_v4\n  release_priority: 5\n  locales: [UNIVERSAL]").unwrap()
        });
        let mut generator = Generator::with_seed(taxonomy, 42);
        let ip = generator.gen_ipv4();
        assert!(ip.split('.').count() == 4);
    }

    #[test]
    fn test_uuid_generation() {
        let taxonomy = Taxonomy::from_yaml("").unwrap_or_else(|_| {
            Taxonomy::from_yaml("cryptographic.uuid:\n  provider: cryptographic\n  method: uuid\n  release_priority: 5\n  locales: [UNIVERSAL]").unwrap()
        });
        let mut generator = Generator::with_seed(taxonomy, 42);
        let uuid = generator.gen_uuid();
        assert!(uuid.len() == 36);
        assert!(uuid.contains('-'));
    }
}
