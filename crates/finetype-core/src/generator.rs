//! Synthetic data generation for all 151 type definitions.
//!
//! Generates synthetic training data using taxonomy keys:
//! `domain.category.type` (e.g., `datetime.timestamp.iso_8601`).
//!
//! Each generator produces strings that match the transformation contract
//! defined in the YAML specification.

use crate::taxonomy::Taxonomy;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
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
        let keys: Vec<String> = self
            .taxonomy
            .at_priority(min_priority)
            .into_iter()
            .map(|(k, _)| k.clone())
            .collect();

        let mut all_samples = Vec::new();

        for key in keys {
            for _ in 0..samples_per_label {
                if let Ok(text) = self.generate_value(&key) {
                    all_samples.push(Sample {
                        text,
                        label: key.clone(),
                    });
                }
            }
        }

        all_samples
    }

    /// Generate a single value for a key (domain.category.type).
    pub fn generate_value(&mut self, key: &str) -> Result<String, GeneratorError> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() != 3 {
            return Err(GeneratorError::UnknownLabel(key.to_string()));
        }

        let (domain, category, type_name) = (parts[0], parts[1], parts[2]);

        match domain {
            "datetime" => self.gen_datetime(category, type_name),
            "technology" => self.gen_technology(category, type_name),
            "identity" => self.gen_identity(category, type_name),
            "geography" => self.gen_geography(category, type_name),
            "representation" => self.gen_representation(category, type_name),
            "container" => self.gen_container(category, type_name),
            _ => Err(GeneratorError::UnknownLabel(key.to_string())),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOMAIN: datetime (46 types)
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_datetime(
        &mut self,
        category: &str,
        type_name: &str,
    ) -> Result<String, GeneratorError> {
        match (category, type_name) {
            // ── timestamp (12 types) ─────────────────────────────────────
            ("timestamp", "iso_8601") => {
                Ok(self.random_datetime().format("%Y-%m-%dT%H:%M:%SZ").to_string())
            }
            ("timestamp", "iso_8601_compact") => {
                Ok(self.random_datetime().format("%Y%m%dT%H%M%S").to_string())
            }
            ("timestamp", "iso_8601_microseconds") => {
                let dt = self.random_datetime();
                let micros = self.rng.gen_range(0..1000000);
                Ok(format!("{}.{:06}Z", dt.format("%Y-%m-%dT%H:%M:%S"), micros))
            }
            ("timestamp", "iso_8601_offset") => {
                let dt = self.random_datetime();
                let offset_h = self.rng.gen_range(-12i32..=12);
                Ok(format!("{}{:+03}:00", dt.format("%Y-%m-%dT%H:%M:%S"), offset_h))
            }
            ("timestamp", "rfc_2822") => {
                Ok(self.random_datetime().format("%a, %d %b %Y %H:%M:%S +0000").to_string())
            }
            ("timestamp", "rfc_2822_ordinal") => {
                let dt = self.random_datetime();
                let day = dt.day();
                let ord = match day % 10 {
                    1 if day != 11 => "st",
                    2 if day != 12 => "nd",
                    3 if day != 13 => "rd",
                    _ => "th",
                };
                Ok(format!("{}{} {} +0000", day, ord, dt.format("%b %Y %H:%M:%S")))
            }
            ("timestamp", "rfc_3339") => {
                let dt = self.random_datetime();
                let offset_h = self.rng.gen_range(-12i32..=12);
                Ok(format!("{}{:+03}:00", dt.format("%Y-%m-%dT%H:%M:%S"), offset_h))
            }
            ("timestamp", "american") => {
                Ok(self.random_datetime().format("%m/%d/%Y %I:%M %p").to_string())
            }
            ("timestamp", "american_24h") => {
                Ok(self.random_datetime().format("%m/%d/%Y %H:%M:%S").to_string())
            }
            ("timestamp", "european") => {
                Ok(self.random_datetime().format("%d/%m/%Y %H:%M").to_string())
            }
            ("timestamp", "iso_microseconds") => {
                let dt = self.random_datetime();
                let micros = self.rng.gen_range(0..1000000);
                Ok(format!("{}.{:06}", dt.format("%Y-%m-%dT%H:%M:%S"), micros))
            }
            ("timestamp", "sql_standard") => {
                Ok(self.random_datetime().format("%Y-%m-%d %H:%M:%S").to_string())
            }

            // ── date (17 types) ──────────────────────────────────────────
            ("date", "iso") => Ok(self.random_datetime().format("%Y-%m-%d").to_string()),
            ("date", "us_slash") => Ok(self.random_datetime().format("%m/%d/%Y").to_string()),
            ("date", "eu_slash") => Ok(self.random_datetime().format("%d/%m/%Y").to_string()),
            ("date", "eu_dot") => Ok(self.random_datetime().format("%d.%m.%Y").to_string()),
            ("date", "compact_ymd") => {
                let dt = self.random_datetime();
                Ok(format!("{}{:02}{:02}", dt.year(), dt.month(), dt.day()))
            }
            ("date", "compact_mdy") => {
                let dt = self.random_datetime();
                Ok(format!("{:02}{:02}{}", dt.month(), dt.day(), dt.year()))
            }
            ("date", "compact_dmy") => {
                let dt = self.random_datetime();
                Ok(format!("{:02}{:02}{}", dt.day(), dt.month(), dt.year()))
            }
            ("date", "short_ymd") => Ok(self.random_datetime().format("%y-%m-%d").to_string()),
            ("date", "short_mdy") => Ok(self.random_datetime().format("%m-%d-%y").to_string()),
            ("date", "short_dmy") => Ok(self.random_datetime().format("%d-%m-%y").to_string()),
            ("date", "abbreviated_month") => {
                Ok(self.random_datetime().format("%b %d, %Y").to_string())
            }
            ("date", "long_full_month") => {
                Ok(self.random_datetime().format("%B %d, %Y").to_string())
            }
            ("date", "weekday_abbreviated_month") => {
                Ok(self.random_datetime().format("%A, %d %b %Y").to_string())
            }
            ("date", "weekday_full_month") => {
                Ok(self.random_datetime().format("%A, %d %B %Y").to_string())
            }
            ("date", "ordinal") => {
                Ok(format!("{}-{:03}", self.rng.gen_range(2020..2030), self.rng.gen_range(1..366)))
            }
            ("date", "julian") => {
                Ok(format!("{:02}-{:03}", self.rng.gen_range(20..30), self.rng.gen_range(1..366)))
            }
            ("date", "iso_week") => {
                Ok(format!("{}-W{:02}", self.rng.gen_range(2020..2030), self.rng.gen_range(1..53)))
            }

            // ── time (5 types) ───────────────────────────────────────────
            ("time", "iso") => {
                let dt = self.random_datetime();
                let micros = self.rng.gen_range(0..1000000);
                Ok(format!("{}.{:06}", dt.format("%H:%M:%S"), micros))
            }
            ("time", "hms_24h") => {
                Ok(self.random_datetime().format("%H:%M:%S").to_string())
            }
            ("time", "hm_24h") => {
                Ok(self.random_datetime().format("%H:%M").to_string())
            }
            ("time", "hms_12h") => {
                Ok(self.random_datetime().format("%I:%M:%S %p").to_string())
            }
            ("time", "hm_12h") => {
                Ok(self.random_datetime().format("%I:%M %p").to_string())
            }

            // ── epoch (3 types) ──────────────────────────────────────────
            ("epoch", "unix_seconds") => {
                Ok(self.rng.gen_range(1_000_000_000i64..2_000_000_000).to_string())
            }
            ("epoch", "unix_milliseconds") => {
                Ok(self.rng.gen_range(1_000_000_000_000i64..2_000_000_000_000).to_string())
            }
            ("epoch", "unix_microseconds") => {
                Ok(self.rng.gen_range(1_000_000_000_000_000i64..2_000_000_000_000_000).to_string())
            }

            // ── offset (2 types) ─────────────────────────────────────────
            ("offset", "utc") => {
                let h = self.rng.gen_range(-12i32..=14);
                Ok(format!("UTC {:+03}:00", h))
            }
            ("offset", "iana") => {
                let tzs = [
                    "America/New_York", "America/Los_Angeles", "America/Chicago",
                    "Europe/London", "Europe/Paris", "Europe/Berlin",
                    "Asia/Tokyo", "Asia/Shanghai", "Asia/Singapore",
                    "Australia/Sydney", "Pacific/Auckland", "Africa/Cairo",
                ];
                Ok(tzs[self.rng.gen_range(0..tzs.len())].to_string())
            }

            // ── duration (1 type) ────────────────────────────────────────
            ("duration", "iso_8601") => {
                let h = self.rng.gen_range(0..24);
                let m = self.rng.gen_range(0..60);
                let s = self.rng.gen_range(0..60);
                if h > 0 {
                    Ok(format!("PT{}H{}M{}S", h, m, s))
                } else if m > 0 {
                    Ok(format!("PT{}M{}S", m, s))
                } else {
                    Ok(format!("PT{}S", s))
                }
            }

            // ── component (6 types) ──────────────────────────────────────
            ("component", "year") => Ok(self.rng.gen_range(1990..2030).to_string()),
            ("component", "month_name") => {
                let months = [
                    "January", "February", "March", "April", "May", "June",
                    "July", "August", "September", "October", "November", "December",
                ];
                Ok(months[self.rng.gen_range(0..12)].to_string())
            }
            ("component", "day_of_month") => Ok(self.rng.gen_range(1u32..=31).to_string()),
            ("component", "day_of_week") => {
                let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
                Ok(days[self.rng.gen_range(0..7)].to_string())
            }
            ("component", "century") => {
                let centuries = ["XVIII", "XIX", "XX", "XXI"];
                Ok(centuries[self.rng.gen_range(0..4)].to_string())
            }
            ("component", "periodicity") => {
                let periods = [
                    "Once", "Daily", "Weekly", "Biweekly",
                    "Monthly", "Quarterly", "Yearly", "Never",
                ];
                Ok(periods[self.rng.gen_range(0..periods.len())].to_string())
            }

            _ => Err(GeneratorError::NotImplemented(format!(
                "datetime.{}.{}", category, type_name
            ))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOMAIN: technology (34 types)
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_technology(
        &mut self,
        category: &str,
        type_name: &str,
    ) -> Result<String, GeneratorError> {
        match (category, type_name) {
            // ── internet (13 types) ──────────────────────────────────────
            ("internet", "ip_v4") => {
                Ok(format!(
                    "{}.{}.{}.{}",
                    self.rng.gen_range(1..255),
                    self.rng.gen_range(0..255),
                    self.rng.gen_range(0..255),
                    self.rng.gen_range(1..255)
                ))
            }
            ("internet", "ip_v4_with_port") => {
                Ok(format!(
                    "{}.{}.{}.{}:{}",
                    self.rng.gen_range(1..255),
                    self.rng.gen_range(0..255),
                    self.rng.gen_range(0..255),
                    self.rng.gen_range(1..255),
                    self.rng.gen_range(1024..65535)
                ))
            }
            ("internet", "ip_v6") => {
                let groups: Vec<String> = (0..8)
                    .map(|_| format!("{:04x}", self.rng.gen_range(0u16..65535)))
                    .collect();
                Ok(groups.join(":"))
            }
            ("internet", "mac_address") => {
                let octets: Vec<String> = (0..6)
                    .map(|_| format!("{:02x}", self.rng.gen::<u8>()))
                    .collect();
                Ok(octets.join(":"))
            }
            ("internet", "url") => {
                let tlds = ["com", "org", "net", "io", "dev", "co", "app"];
                let words: Vec<String> = (0..self.rng.gen_range(1..3))
                    .map(|_| self.random_word())
                    .collect();
                let domain = words.join("");
                let tld = tlds[self.rng.gen_range(0..tlds.len())];
                let path_segments: Vec<String> = (0..self.rng.gen_range(1..4))
                    .map(|_| self.random_word())
                    .collect();
                Ok(format!("https://{}.{}/{}", domain, tld, path_segments.join("/")))
            }
            ("internet", "uri") => {
                let schemes = ["https", "http", "ftp", "mailto", "ssh"];
                let scheme = schemes[self.rng.gen_range(0..schemes.len())];
                if scheme == "mailto" {
                    Ok(format!("mailto:{}@{}.com", self.random_word(), self.random_word()))
                } else {
                    Ok(format!("{}://{}.com/{}", scheme, self.random_word(), self.random_word()))
                }
            }
            ("internet", "hostname") => {
                let tlds = ["com", "org", "net", "io", "dev"];
                Ok(format!("{}.{}", self.random_word(), tlds[self.rng.gen_range(0..tlds.len())]))
            }
            ("internet", "port") => Ok(self.rng.gen_range(1..65535).to_string()),
            ("internet", "top_level_domain") => {
                let tlds = ["com", "org", "net", "io", "dev", "edu", "gov", "mil", "co.uk", "com.au"];
                Ok(tlds[self.rng.gen_range(0..tlds.len())].to_string())
            }
            ("internet", "slug") => {
                let words: Vec<String> = (0..self.rng.gen_range(2..6))
                    .map(|_| self.random_word())
                    .collect();
                Ok(words.join("-"))
            }
            ("internet", "user_agent") => {
                let agents = [
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
                    "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
                    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
                    "curl/8.4.0",
                    "python-requests/2.31.0",
                    "Go-http-client/2.0",
                ];
                Ok(agents[self.rng.gen_range(0..agents.len())].to_string())
            }
            ("internet", "http_method") => {
                let methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
                Ok(methods[self.rng.gen_range(0..methods.len())].to_string())
            }
            ("internet", "http_status_code") => {
                let codes = [200, 201, 204, 301, 302, 304, 400, 401, 403, 404, 405, 500, 502, 503];
                Ok(codes[self.rng.gen_range(0..codes.len())].to_string())
            }

            // ── cryptographic (4 types) ──────────────────────────────────
            ("cryptographic", "uuid") => Ok(Uuid::new_v4().to_string()),
            ("cryptographic", "hash") => {
                // Generate MD5 (32), SHA1 (40), or SHA256 (64) length hashes
                let lengths = [32, 40, 64];
                let len = lengths[self.rng.gen_range(0..3)];
                Ok(self.gen_hex_string(len))
            }
            ("cryptographic", "token_hex") => {
                let len = self.rng.gen_range(16..48);
                Ok(self.gen_hex_string(len))
            }
            ("cryptographic", "token_urlsafe") => {
                use rand::distributions::Alphanumeric;
                let len = self.rng.gen_range(22..44);
                let token: String = (&mut self.rng)
                    .sample_iter(Alphanumeric)
                    .take(len)
                    .map(|b| b as char)
                    .collect();
                Ok(token.replace('+', "-").replace('/', "_"))
            }

            // ── code (6 types) ───────────────────────────────────────────
            ("code", "isbn") => {
                let prefix = if self.rng.gen_bool(0.8) { "978" } else { "979" };
                let group = self.rng.gen_range(0..9);
                let publisher = self.rng.gen_range(10000..99999);
                let title = self.rng.gen_range(100..999);
                let check = self.rng.gen_range(0..9);
                Ok(format!("{}-{}-{:05}-{:03}-{}", prefix, group, publisher, title, check))
            }
            ("code", "imei") => {
                Ok(format!("{:015}", self.rng.gen_range(100_000_000_000_000u64..999_999_999_999_999)))
            }
            ("code", "ean") => {
                if self.rng.gen_bool(0.7) {
                    Ok(format!("{:013}", self.rng.gen_range(1_000_000_000_000u64..9_999_999_999_999)))
                } else {
                    Ok(format!("{:08}", self.rng.gen_range(10_000_000u64..99_999_999)))
                }
            }
            ("code", "issn") => {
                let check_chars = "0123456789X";
                let check = check_chars.chars().nth(self.rng.gen_range(0..11)).unwrap();
                Ok(format!("{:04}-{:03}{}", self.rng.gen_range(1000..9999), self.rng.gen_range(100..999), check))
            }
            ("code", "locale_code") => {
                let codes = [
                    "en", "en-US", "en-GB", "en-AU", "en-CA",
                    "fr", "fr-FR", "fr-CA", "de", "de-DE", "de-AT",
                    "es", "es-ES", "es-MX", "it", "it-IT",
                    "ja", "ja-JP", "ko", "ko-KR", "zh", "zh-CN", "zh-TW",
                    "pt", "pt-BR", "ru", "ru-RU", "nl", "nl-NL",
                ];
                Ok(codes[self.rng.gen_range(0..codes.len())].to_string())
            }
            ("code", "pin") => {
                let len = if self.rng.gen_bool(0.7) { 4 } else { 6 };
                Ok(format!("{:0width$}", self.rng.gen_range(0..10u32.pow(len)), width = len as usize))
            }

            // ── development (8 types) ────────────────────────────────────
            ("development", "version") => {
                let major = self.rng.gen_range(0..20);
                let minor = self.rng.gen_range(0..50);
                let patch = self.rng.gen_range(0..100);
                let prefix = if self.rng.gen_bool(0.3) { "v" } else { "" };
                let pre = if self.rng.gen_bool(0.2) {
                    let tags = ["-alpha", "-beta", "-rc.1", "-dev"];
                    tags[self.rng.gen_range(0..tags.len())]
                } else {
                    ""
                };
                Ok(format!("{}{}.{}.{}{}", prefix, major, minor, patch, pre))
            }
            ("development", "calver") => {
                let y = self.rng.gen_range(2020..2026);
                let m = self.rng.gen_range(1..13);
                if self.rng.gen_bool(0.5) {
                    let d = self.rng.gen_range(1..29);
                    Ok(format!("{}.{:02}.{:02}", y, m, d))
                } else {
                    Ok(format!("{}.{:02}", y, m))
                }
            }
            ("development", "programming_language") => {
                let langs = [
                    "Python", "JavaScript", "TypeScript", "Java", "C++", "C#", "Go",
                    "Rust", "PHP", "Ruby", "Kotlin", "Swift", "Scala", "Haskell",
                    "R", "Julia", "MATLAB", "Perl", "Lua", "Elixir",
                ];
                Ok(langs[self.rng.gen_range(0..langs.len())].to_string())
            }
            ("development", "software_license") => {
                let licenses = [
                    "MIT", "Apache-2.0", "GPL-3.0", "GPL-2.0", "BSD-3-Clause",
                    "BSD-2-Clause", "ISC", "MPL-2.0", "LGPL-3.0", "AGPL-3.0",
                    "Unlicense", "CC0-1.0",
                ];
                Ok(licenses[self.rng.gen_range(0..licenses.len())].to_string())
            }
            ("development", "stage") => {
                let stages = ["Alpha", "Beta", "Release Candidate", "Stable", "LTS", "Deprecated"];
                Ok(stages[self.rng.gen_range(0..stages.len())].to_string())
            }
            ("development", "os") => {
                let oses = [
                    "Windows 10", "Windows 11", "macOS", "Ubuntu", "Fedora",
                    "Debian", "Arch Linux", "CentOS", "iOS", "Android",
                ];
                Ok(oses[self.rng.gen_range(0..oses.len())].to_string())
            }
            ("development", "boolean") => {
                let bools = ["true", "false", "yes", "no", "1", "0", "True", "False", "YES", "NO"];
                Ok(bools[self.rng.gen_range(0..bools.len())].to_string())
            }

            // ── hardware (4 types) ───────────────────────────────────────
            ("hardware", "cpu") => {
                let cpus = [
                    "Intel Core i9-14900K", "Intel Core i7-14700K", "Intel Core i5-14600K",
                    "AMD Ryzen 9 7950X", "AMD Ryzen 7 7700X", "AMD Ryzen 5 7600X",
                    "Apple M3 Pro", "Apple M3 Max", "Apple M2 Ultra",
                    "Qualcomm Snapdragon 8 Gen 3",
                ];
                Ok(cpus[self.rng.gen_range(0..cpus.len())].to_string())
            }
            ("hardware", "ram_size") => {
                let sizes = ["4GB", "8GB", "16GB", "32GB", "64GB", "128GB", "256GB", "512MB"];
                Ok(sizes[self.rng.gen_range(0..sizes.len())].to_string())
            }
            ("hardware", "screen_size") => {
                let sizes = ["13.3\"", "14\"", "15.6\"", "16\"", "24\"", "27\"", "32\"", "34\""];
                Ok(sizes[self.rng.gen_range(0..sizes.len())].to_string())
            }
            ("hardware", "generation") => {
                let gens = [
                    "1st Generation", "2nd Generation", "3rd Generation",
                    "4th Generation", "5th Generation", "Gen 3", "Gen 4", "Gen 5",
                    "Rev 2", "v3",
                ];
                Ok(gens[self.rng.gen_range(0..gens.len())].to_string())
            }

            _ => Err(GeneratorError::NotImplemented(format!(
                "technology.{}.{}", category, type_name
            ))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOMAIN: identity (25 types)
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_identity(
        &mut self,
        category: &str,
        type_name: &str,
    ) -> Result<String, GeneratorError> {
        match (category, type_name) {
            // ── person (16 types) ────────────────────────────────────────
            ("person", "full_name") => {
                let first = self.random_first_name();
                let last = self.random_last_name();
                Ok(format!("{} {}", first, last))
            }
            ("person", "first_name") => Ok(self.random_first_name()),
            ("person", "last_name") => Ok(self.random_last_name()),
            ("person", "email") => {
                let first = self.random_first_name().to_lowercase();
                let last = self.random_last_name().to_lowercase();
                let domains = ["gmail.com", "yahoo.com", "outlook.com", "example.com", "company.org"];
                let sep = [".", "_", ""][self.rng.gen_range(0..3)];
                let num = if self.rng.gen_bool(0.3) {
                    self.rng.gen_range(1..99).to_string()
                } else {
                    String::new()
                };
                Ok(format!("{}{}{}{}@{}", first, sep, last, num, domains[self.rng.gen_range(0..domains.len())]))
            }
            ("person", "phone_number") => {
                let formats = [
                    format!("+1 ({:03}) {:03}-{:04}", self.rng.gen_range(200..999), self.rng.gen_range(200..999), self.rng.gen_range(1000..9999)),
                    format!("{:03}-{:03}-{:04}", self.rng.gen_range(200..999), self.rng.gen_range(200..999), self.rng.gen_range(1000..9999)),
                    format!("+44 {:02} {:04} {:04}", self.rng.gen_range(20..79), self.rng.gen_range(1000..9999), self.rng.gen_range(1000..9999)),
                    format!("+61 {:01} {:04} {:04}", self.rng.gen_range(2..9), self.rng.gen_range(1000..9999), self.rng.gen_range(1000..9999)),
                    format!("+33 {:01} {:02} {:02} {:02} {:02}", self.rng.gen_range(1..9), self.rng.gen_range(10..99), self.rng.gen_range(10..99), self.rng.gen_range(10..99), self.rng.gen_range(10..99)),
                ];
                Ok(formats[self.rng.gen_range(0..formats.len())].clone())
            }
            ("person", "username") => {
                let first = self.random_first_name().to_lowercase();
                let seps = [".", "_", "-", ""];
                let sep = seps[self.rng.gen_range(0..seps.len())];
                let suffix = if self.rng.gen_bool(0.5) {
                    self.rng.gen_range(1..999).to_string()
                } else {
                    self.random_word()
                };
                Ok(format!("{}{}{}", first, sep, suffix))
            }
            ("person", "password") => {
                use rand::distributions::Alphanumeric;
                let len = self.rng.gen_range(8..20);
                let mut pass: String = (&mut self.rng)
                    .sample_iter(Alphanumeric)
                    .take(len)
                    .map(|b| b as char)
                    .collect();
                // Add special chars
                let specials = "!@#$%^&*()_+-=[]{}|;:',.<>?";
                let pos = self.rng.gen_range(0..pass.len());
                let special = specials.chars().nth(self.rng.gen_range(0..specials.len())).unwrap();
                pass.insert(pos, special);
                Ok(pass)
            }
            ("person", "gender") => {
                let genders = ["Male", "Female", "Non-binary", "Other", "Prefer not to say"];
                Ok(genders[self.rng.gen_range(0..genders.len())].to_string())
            }
            ("person", "gender_code") => {
                let codes = ["M", "F", "X"];
                Ok(codes[self.rng.gen_range(0..codes.len())].to_string())
            }
            ("person", "gender_symbol") => {
                let symbols = ["♂", "♀", "⚧"];
                Ok(symbols[self.rng.gen_range(0..symbols.len())].to_string())
            }
            ("person", "nationality") => {
                let nationalities = [
                    "American", "British", "Canadian", "Australian", "German",
                    "French", "Japanese", "Chinese", "Korean", "Brazilian",
                    "Indian", "Mexican", "Italian", "Spanish", "Russian",
                ];
                Ok(nationalities[self.rng.gen_range(0..nationalities.len())].to_string())
            }
            ("person", "blood_type") => {
                let types = ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"];
                Ok(types[self.rng.gen_range(0..types.len())].to_string())
            }
            ("person", "height") => {
                if self.rng.gen_bool(0.6) {
                    // Metric
                    Ok(format!("{} cm", self.rng.gen_range(150..200)))
                } else {
                    // Imperial
                    let feet = self.rng.gen_range(5..7);
                    let inches = self.rng.gen_range(0..12);
                    Ok(format!("{}'{:02}\"", feet, inches))
                }
            }
            ("person", "weight") => {
                if self.rng.gen_bool(0.6) {
                    Ok(format!("{} kg", self.rng.gen_range(45..120)))
                } else {
                    Ok(format!("{} lbs", self.rng.gen_range(100..265)))
                }
            }
            ("person", "age") => Ok(self.rng.gen_range(1..100).to_string()),
            ("person", "occupation") => {
                let jobs = [
                    "Software Engineer", "Data Scientist", "Product Manager",
                    "Designer", "Teacher", "Nurse", "Accountant", "Lawyer",
                    "Chef", "Pilot", "Architect", "Pharmacist",
                    "Marketing Manager", "Sales Representative", "Researcher",
                ];
                Ok(jobs[self.rng.gen_range(0..jobs.len())].to_string())
            }
            // ── payment (7 types) ────────────────────────────────────────
            ("payment", "credit_card_number") => {
                // Generate valid-looking card numbers (Luhn not enforced, pattern only)
                let prefixes = ["4", "51", "52", "53", "54", "55", "34", "37"];
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let remaining = 16 - prefix.len();
                let suffix: String = (0..remaining)
                    .map(|_| (b'0' + self.rng.gen_range(0..10)) as char)
                    .collect();
                Ok(format!("{}{}", prefix, suffix))
            }
            ("payment", "credit_card_expiration_date") => {
                let month = self.rng.gen_range(1..13);
                let year = self.rng.gen_range(25..32);
                Ok(format!("{:02}/{:02}", month, year))
            }
            ("payment", "cvv") => {
                if self.rng.gen_bool(0.85) {
                    Ok(format!("{:03}", self.rng.gen_range(100..999)))
                } else {
                    // Amex 4-digit CID
                    Ok(format!("{:04}", self.rng.gen_range(1000..9999)))
                }
            }
            ("payment", "credit_card_network") => {
                let networks = ["Visa", "Mastercard", "Amex", "Discover", "Diners Club", "JCB"];
                Ok(networks[self.rng.gen_range(0..networks.len())].to_string())
            }
            ("payment", "bitcoin_address") => {
                let base58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
                let prefix_choice = self.rng.gen_range(0..3);
                match prefix_choice {
                    0 => {
                        // P2PKH (1...)
                        let chars: String = (0..33)
                            .map(|_| base58.chars().nth(self.rng.gen_range(0..58)).unwrap())
                            .collect();
                        Ok(format!("1{}", chars))
                    }
                    1 => {
                        // P2SH (3...)
                        let chars: String = (0..33)
                            .map(|_| base58.chars().nth(self.rng.gen_range(0..58)).unwrap())
                            .collect();
                        Ok(format!("3{}", chars))
                    }
                    _ => {
                        // Bech32 (bc1...)
                        let bech32_chars = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
                        let chars: String = (0..39)
                            .map(|_| bech32_chars.chars().nth(self.rng.gen_range(0..32)).unwrap())
                            .collect();
                        Ok(format!("bc1{}", chars))
                    }
                }
            }
            ("payment", "ethereum_address") => {
                Ok(format!("0x{}", self.gen_hex_string(40)))
            }
            ("payment", "paypal_email") => {
                let first = self.random_first_name().to_lowercase();
                let last = self.random_last_name().to_lowercase();
                let domains = ["gmail.com", "yahoo.com", "hotmail.com", "outlook.com"];
                Ok(format!("{}.{}@{}", first, last, domains[self.rng.gen_range(0..domains.len())]))
            }

            // ── academic (2 types) ───────────────────────────────────────
            ("academic", "degree") => {
                let degrees = [
                    "Bachelor of Science", "Bachelor of Arts", "Master of Science",
                    "Master of Arts", "Master of Business Administration",
                    "Doctor of Philosophy", "Associate Degree",
                    "Juris Doctor", "Doctor of Medicine",
                ];
                Ok(degrees[self.rng.gen_range(0..degrees.len())].to_string())
            }
            ("academic", "university") => {
                let unis = [
                    "Harvard University", "Stanford University", "MIT",
                    "Oxford University", "Cambridge University",
                    "ETH Zurich", "University of Tokyo", "Caltech",
                    "Princeton University", "Yale University",
                    "Columbia University", "UC Berkeley",
                    "University of Melbourne", "Sorbonne University",
                ];
                Ok(unis[self.rng.gen_range(0..unis.len())].to_string())
            }

            _ => Err(GeneratorError::NotImplemented(format!(
                "identity.{}.{}", category, type_name
            ))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOMAIN: geography (16 types)
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_geography(
        &mut self,
        category: &str,
        type_name: &str,
    ) -> Result<String, GeneratorError> {
        match (category, type_name) {
            // ── location (5 types) ───────────────────────────────────────
            ("location", "country") => {
                let countries = [
                    "United States", "United Kingdom", "Canada", "Australia",
                    "Germany", "France", "Japan", "China", "India", "Brazil",
                    "Mexico", "Italy", "Spain", "South Korea", "Russia",
                    "Netherlands", "Switzerland", "Sweden", "Norway", "Denmark",
                ];
                Ok(countries[self.rng.gen_range(0..countries.len())].to_string())
            }
            ("location", "country_code") => {
                let codes = [
                    "US", "GB", "CA", "AU", "DE", "FR", "JP", "CN", "IN", "BR",
                    "MX", "IT", "ES", "KR", "RU", "NL", "CH", "SE", "NO", "DK",
                ];
                Ok(codes[self.rng.gen_range(0..codes.len())].to_string())
            }
            ("location", "continent") => {
                let continents = [
                    "Africa", "Asia", "Europe", "North America",
                    "South America", "Oceania", "Antarctica",
                ];
                Ok(continents[self.rng.gen_range(0..continents.len())].to_string())
            }
            ("location", "region") => {
                let regions = [
                    "California", "Texas", "New York", "Florida", "Ontario",
                    "Quebec", "Bavaria", "Île-de-France", "New South Wales",
                    "Victoria", "Tokyo", "Guangdong", "Maharashtra",
                ];
                Ok(regions[self.rng.gen_range(0..regions.len())].to_string())
            }
            ("location", "city") => {
                let cities = [
                    "New York", "London", "Tokyo", "Paris", "Sydney",
                    "Berlin", "Toronto", "San Francisco", "Shanghai",
                    "Mumbai", "São Paulo", "Seoul", "Singapore",
                    "Amsterdam", "Barcelona", "Dubai", "Los Angeles",
                ];
                Ok(cities[self.rng.gen_range(0..cities.len())].to_string())
            }

            // ── address (5 types) ────────────────────────────────────────
            ("address", "full_address") => {
                let num = self.rng.gen_range(1..9999);
                let streets = ["Main Street", "Oak Avenue", "Park Road", "Broadway", "Elm Street", "5th Avenue", "High Street", "King Street"];
                let cities = ["New York", "London", "Paris", "Berlin", "Sydney", "Toronto", "Tokyo"];
                let codes = ["NY 10001", "W1C 1AX", "75004", "10115", "2000", "M5V 2T6", "100-0001"];
                let idx = self.rng.gen_range(0..streets.len().min(cities.len()).min(codes.len()));
                Ok(format!("{} {}, {}, {}", num, streets[idx], cities[idx], codes[idx]))
            }
            ("address", "street_number") => {
                if self.rng.gen_bool(0.9) {
                    Ok(self.rng.gen_range(1..9999).to_string())
                } else {
                    let suffix = ['A', 'B', 'C'][self.rng.gen_range(0..3)];
                    Ok(format!("{}{}", self.rng.gen_range(1..999), suffix))
                }
            }
            ("address", "street_name") => {
                let names = [
                    "Main Street", "Oak Avenue", "Elm Street", "Park Road",
                    "Broadway", "5th Avenue", "High Street", "King Street",
                    "Queen Street", "Church Street", "Maple Drive", "Cedar Lane",
                ];
                Ok(names[self.rng.gen_range(0..names.len())].to_string())
            }
            ("address", "street_suffix") => {
                let suffixes = [
                    "Street", "Avenue", "Boulevard", "Road", "Lane",
                    "Drive", "Court", "Way", "Circle", "Place",
                    "St", "Ave", "Blvd", "Rd", "Ln", "Dr",
                ];
                Ok(suffixes[self.rng.gen_range(0..suffixes.len())].to_string())
            }
            ("address", "postal_code") => {
                let formats = [
                    format!("{:05}", self.rng.gen_range(10000..99999)),  // US ZIP
                    format!("{:05}", self.rng.gen_range(10000..99999)),  // US ZIP
                    format!("{}{}{} {}{}{}",
                        (b'A' + self.rng.gen_range(0..26)) as char,
                        self.rng.gen_range(0..9),
                        (b'A' + self.rng.gen_range(0..26)) as char,
                        self.rng.gen_range(0..9),
                        (b'A' + self.rng.gen_range(0..26)) as char,
                        self.rng.gen_range(0..9),
                    ),  // UK postcode
                    format!("{:05}", self.rng.gen_range(10000..99999)),  // DE/FR
                ];
                Ok(formats[self.rng.gen_range(0..formats.len())].clone())
            }

            // ── coordinate (3 types) ─────────────────────────────────────
            ("coordinate", "latitude") => {
                let lat = (self.rng.gen::<f64>() - 0.5) * 180.0;
                Ok(format!("{:.4}", lat))
            }
            ("coordinate", "longitude") => {
                let lon = (self.rng.gen::<f64>() - 0.5) * 360.0;
                Ok(format!("{:.4}", lon))
            }
            ("coordinate", "coordinates") => {
                let lat = (self.rng.gen::<f64>() - 0.5) * 180.0;
                let lon = (self.rng.gen::<f64>() - 0.5) * 360.0;
                Ok(format!("{:.4},{:.4}", lat, lon))
            }

            // ── transportation (2 types) ─────────────────────────────────
            ("transportation", "iata_code") => {
                let code: String = (0..3)
                    .map(|_| (b'A' + self.rng.gen_range(0..26)) as char)
                    .collect();
                Ok(code)
            }
            ("transportation", "icao_code") => {
                let code: String = (0..4)
                    .map(|_| (b'A' + self.rng.gen_range(0..26)) as char)
                    .collect();
                Ok(code)
            }

            // ── contact (1 type) ─────────────────────────────────────────
            ("contact", "calling_code") => {
                let codes = ["+1", "+44", "+33", "+49", "+81", "+86", "+91", "+61", "+55", "+82"];
                Ok(codes[self.rng.gen_range(0..codes.len())].to_string())
            }

            _ => Err(GeneratorError::NotImplemented(format!(
                "geography.{}.{}", category, type_name
            ))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOMAIN: representation (19 types)
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_representation(
        &mut self,
        category: &str,
        type_name: &str,
    ) -> Result<String, GeneratorError> {
        match (category, type_name) {
            // ── numeric (5 types) ────────────────────────────────────────
            ("numeric", "integer_number") => {
                Ok(self.rng.gen_range(-100000i64..100000).to_string())
            }
            ("numeric", "decimal_number") => {
                let val = (self.rng.gen::<f64>() - 0.5) * 2000.0;
                let precision = self.rng.gen_range(1..8);
                Ok(format!("{:.prec$}", val, prec = precision))
            }
            ("numeric", "scientific_notation") => {
                let mantissa = self.rng.gen::<f64>() * 9.0 + 1.0;
                let exponent = self.rng.gen_range(-15i32..15);
                let e_char = if self.rng.gen_bool(0.5) { 'e' } else { 'E' };
                Ok(format!("{:.2}{}{:+}", mantissa, e_char, exponent))
            }
            ("numeric", "percentage") => {
                let val = self.rng.gen::<f64>() * 100.0;
                if self.rng.gen_bool(0.7) {
                    Ok(format!("{:.1}%", val))
                } else {
                    Ok(format!("{:.2}%", val))
                }
            }
            ("numeric", "increment") => {
                Ok(self.rng.gen_range(1..100000).to_string())
            }

            // ── text (5 types) ───────────────────────────────────────────
            ("text", "plain_text") => {
                let words: Vec<String> = (0..self.rng.gen_range(5..25))
                    .map(|_| self.random_word())
                    .collect();
                Ok(words.join(" "))
            }
            ("text", "sentence") => {
                let mut words: Vec<String> = (0..self.rng.gen_range(5..15))
                    .map(|_| self.random_word())
                    .collect();
                // Capitalize first word
                if let Some(first) = words.first_mut() {
                    let mut chars = first.chars();
                    if let Some(c) = chars.next() {
                        *first = c.to_uppercase().collect::<String>() + chars.as_str();
                    }
                }
                let ending = [".", "!", "?"][self.rng.gen_range(0..3)];
                Ok(format!("{}{}", words.join(" "), ending))
            }
            ("text", "word") => Ok(self.random_word()),
            ("text", "color_hex") => {
                let r = self.rng.gen::<u8>();
                let g = self.rng.gen::<u8>();
                let b = self.rng.gen::<u8>();
                if self.rng.gen_bool(0.8) {
                    Ok(format!("#{:02X}{:02X}{:02X}", r, g, b))
                } else {
                    Ok(format!("{:02x}{:02x}{:02x}", r, g, b))
                }
            }
            ("text", "color_rgb") => {
                let r = self.rng.gen_range(0..256);
                let g = self.rng.gen_range(0..256);
                let b = self.rng.gen_range(0..256);
                if self.rng.gen_bool(0.6) {
                    Ok(format!("rgb({}, {}, {})", r, g, b))
                } else {
                    Ok(format!("{}, {}, {}", r, g, b))
                }
            }
            ("text", "emoji") => {
                let emojis = [
                    "\u{1f600}", "\u{1f602}", "\u{1f923}", "\u{1f60a}", "\u{1f60d}", "\u{1f970}", "\u{1f60e}", "\u{1f914}",
                    "\u{1f622}", "\u{1f621}", "\u{1f389}", "\u{1f525}", "\u{2764}\u{fe0f}", "\u{1f44d}", "\u{1f44e}", "\u{1f680}",
                    "\u{1f4bb}", "\u{1f4f1}", "\u{1f30d}", "\u{26a1}", "\u{2705}", "\u{274c}", "\u{2b50}", "\u{1f3b8}",
                ];
                Ok(emojis[self.rng.gen_range(0..emojis.len())].to_string())
            }

            // ── file (3 types) ───────────────────────────────────────────
            ("file", "extension") => {
                let exts = [
                    "txt", "pdf", "docx", "xlsx", "csv", "json", "xml", "html",
                    "js", "py", "rs", "go", "java", "cpp", "md", "yaml",
                    "png", "jpg", "gif", "svg", "mp4", "mp3", "zip", "gz",
                ];
                Ok(exts[self.rng.gen_range(0..exts.len())].to_string())
            }
            ("file", "mime_type") => {
                let types = [
                    "text/plain", "text/html", "text/css", "text/csv",
                    "application/json", "application/xml", "application/pdf",
                    "application/javascript", "application/octet-stream",
                    "image/png", "image/jpeg", "image/gif", "image/svg+xml",
                    "audio/mpeg", "audio/wav", "video/mp4", "video/webm",
                    "multipart/form-data",
                ];
                Ok(types[self.rng.gen_range(0..types.len())].to_string())
            }
            ("file", "file_size") => {
                let units = ["B", "KB", "MB", "GB"];
                let unit = units[self.rng.gen_range(0..units.len())];
                let size = if unit == "B" {
                    self.rng.gen_range(1..1024).to_string()
                } else if self.rng.gen_bool(0.5) {
                    format!("{:.1}", self.rng.gen::<f64>() * 999.0 + 0.1)
                } else {
                    self.rng.gen_range(1..999).to_string()
                };
                Ok(format!("{} {}", size, unit))
            }

            // ── scientific (5 types) ─────────────────────────────────────
            ("scientific", "dna_sequence") => {
                let len = self.rng.gen_range(8..30);
                let bases = ['A', 'T', 'G', 'C'];
                let seq: String = (0..len)
                    .map(|_| bases[self.rng.gen_range(0..4)])
                    .collect();
                Ok(seq)
            }
            ("scientific", "rna_sequence") => {
                let len = self.rng.gen_range(8..30);
                let bases = ['A', 'U', 'G', 'C'];
                let seq: String = (0..len)
                    .map(|_| bases[self.rng.gen_range(0..4)])
                    .collect();
                Ok(seq)
            }
            ("scientific", "protein_sequence") => {
                let len = self.rng.gen_range(10..50);
                let amino = "ACDEFGHIKLMNPQRSTVWY";
                let seq: String = (0..len)
                    .map(|_| amino.chars().nth(self.rng.gen_range(0..20)).unwrap())
                    .collect();
                Ok(seq)
            }
            ("scientific", "measurement_unit") => {
                let units = [
                    "meter", "kilogram", "second", "ampere", "kelvin", "mole", "candela",
                    "hertz", "newton", "joule", "watt", "pascal", "liter", "gram",
                    "m", "kg", "s", "A", "K", "mol", "cd", "Hz", "N", "J", "W", "Pa", "L", "g",
                ];
                Ok(units[self.rng.gen_range(0..units.len())].to_string())
            }
            ("scientific", "metric_prefix") => {
                let prefixes = [
                    "yotta", "zetta", "exa", "peta", "tera", "giga", "mega", "kilo",
                    "hecto", "deca", "deci", "centi", "milli", "micro", "nano",
                    "pico", "femto", "atto",
                ];
                Ok(prefixes[self.rng.gen_range(0..prefixes.len())].to_string())
            }

            _ => Err(GeneratorError::NotImplemented(format!(
                "representation.{}.{}", category, type_name
            ))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOMAIN: container (11 types)
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_container(
        &mut self,
        category: &str,
        type_name: &str,
    ) -> Result<String, GeneratorError> {
        match (category, type_name) {
            // ── object (5 types) ─────────────────────────────────────────
            ("object", "json") => {
                let templates = [
                    format!(
                        r#"{{"name":"{}","age":{},"active":{}}}"#,
                        self.random_first_name(),
                        self.rng.gen_range(18..80),
                        self.rng.gen_bool(0.7)
                    ),
                    format!(
                        r#"{{"id":{},"email":"{}@{}.com","role":"{}"}}"#,
                        self.rng.gen_range(1..10000),
                        self.random_first_name().to_lowercase(),
                        self.random_word(),
                        ["admin", "user", "moderator"][self.rng.gen_range(0..3)]
                    ),
                    format!(
                        r#"{{"product":"{}","price":{:.2},"currency":"{}"}}"#,
                        self.random_word(),
                        self.rng.gen::<f64>() * 999.0 + 0.01,
                        ["USD", "EUR", "GBP", "JPY"][self.rng.gen_range(0..4)]
                    ),
                    format!(
                        r#"{{"lat":{:.4},"lon":{:.4},"label":"{}"}}"#,
                        (self.rng.gen::<f64>() - 0.5) * 180.0,
                        (self.rng.gen::<f64>() - 0.5) * 360.0,
                        self.random_word()
                    ),
                ];
                Ok(templates[self.rng.gen_range(0..templates.len())].clone())
            }
            ("object", "json_array") => {
                let templates = [
                    format!(
                        "[{},{},{}]",
                        self.rng.gen_range(1..100),
                        self.rng.gen_range(1..100),
                        self.rng.gen_range(1..100)
                    ),
                    format!(
                        r#"["{}","{}","{}"]"#,
                        self.random_word(),
                        self.random_word(),
                        self.random_word()
                    ),
                    format!(
                        r#"[{{"id":{},"name":"{}"}},{{"id":{},"name":"{}"}}]"#,
                        self.rng.gen_range(1..100),
                        self.random_first_name(),
                        self.rng.gen_range(1..100),
                        self.random_first_name()
                    ),
                ];
                Ok(templates[self.rng.gen_range(0..templates.len())].clone())
            }
            ("object", "xml") => {
                let name = self.random_first_name();
                let age = self.rng.gen_range(18..80);
                let templates = [
                    format!("<person><name>{}</name><age>{}</age></person>", name, age),
                    format!(
                        "<item id=\"{}\"><title>{}</title><price>{:.2}</price></item>",
                        self.rng.gen_range(1..1000),
                        self.random_word(),
                        self.rng.gen::<f64>() * 100.0
                    ),
                    format!(
                        "<record><field name=\"status\">{}</field></record>",
                        ["active", "inactive", "pending"][self.rng.gen_range(0..3)]
                    ),
                ];
                Ok(templates[self.rng.gen_range(0..templates.len())].clone())
            }
            ("object", "yaml") => {
                let templates = [
                    format!(
                        "name: {}\nage: {}\nactive: {}",
                        self.random_first_name(),
                        self.rng.gen_range(18..80),
                        self.rng.gen_bool(0.7)
                    ),
                    format!(
                        "server:\n  host: {}.com\n  port: {}\n  ssl: true",
                        self.random_word(),
                        self.rng.gen_range(80..9000)
                    ),
                    format!(
                        "database:\n  driver: {}\n  name: {}",
                        ["postgres", "mysql", "sqlite"][self.rng.gen_range(0..3)],
                        self.random_word()
                    ),
                ];
                Ok(templates[self.rng.gen_range(0..templates.len())].clone())
            }
            ("object", "csv") => {
                let templates = [
                    format!(
                        "{},{},{},{}",
                        self.random_first_name(),
                        self.rng.gen_range(18..80),
                        self.random_first_name().to_lowercase() + "@example.com",
                        ["active", "inactive"][self.rng.gen_range(0..2)]
                    ),
                    format!(
                        "{},{:.2},{},{}",
                        self.random_word(),
                        self.rng.gen::<f64>() * 100.0,
                        self.rng.gen_range(1..1000),
                        ["USD", "EUR", "GBP"][self.rng.gen_range(0..3)]
                    ),
                ];
                Ok(templates[self.rng.gen_range(0..templates.len())].clone())
            }

            // ── array (4 types) ──────────────────────────────────────────
            ("array", "comma_separated") => {
                let count = self.rng.gen_range(3..8);
                if self.rng.gen_bool(0.5) {
                    // Words
                    let items: Vec<String> = (0..count).map(|_| self.random_word()).collect();
                    Ok(items.join(","))
                } else {
                    // Numbers
                    let items: Vec<String> = (0..count)
                        .map(|_| self.rng.gen_range(1..100).to_string())
                        .collect();
                    Ok(items.join(","))
                }
            }
            ("array", "pipe_separated") => {
                let count = self.rng.gen_range(3..8);
                let items: Vec<String> = (0..count).map(|_| self.random_word()).collect();
                Ok(items.join("|"))
            }
            ("array", "semicolon_separated") => {
                let count = self.rng.gen_range(3..8);
                let items: Vec<String> = (0..count).map(|_| self.random_word()).collect();
                Ok(items.join(";"))
            }
            ("array", "whitespace_separated") => {
                let count = self.rng.gen_range(3..8);
                let items: Vec<String> = (0..count).map(|_| self.random_word()).collect();
                if self.rng.gen_bool(0.7) {
                    Ok(items.join(" "))
                } else {
                    Ok(items.join("\t"))
                }
            }

            // ── key_value (2 types) ──────────────────────────────────────
            ("key_value", "query_string") => {
                let count = self.rng.gen_range(2..5);
                let pairs: Vec<String> = (0..count)
                    .map(|_| format!("{}={}", self.random_word(), self.random_word()))
                    .collect();
                Ok(pairs.join("&"))
            }
            ("key_value", "form_data") => {
                let fields = [
                    ("username", self.random_first_name().to_lowercase()),
                    ("email", format!("{}@example.com", self.random_word())),
                    ("password", format!("pass{}", self.rng.gen_range(1000..9999))),
                ];
                let count = self.rng.gen_range(2..=3);
                let pairs: Vec<String> = fields[..count]
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();
                Ok(pairs.join("&"))
            }

            _ => Err(GeneratorError::NotImplemented(format!(
                "container.{}.{}", category, type_name
            ))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SHARED HELPERS
    // ═══════════════════════════════════════════════════════════════════════════

    fn random_datetime(&mut self) -> NaiveDateTime {
        let year = self.rng.gen_range(2015..2030);
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

    fn gen_hex_string(&mut self, char_count: usize) -> String {
        (0..char_count / 2)
            .map(|_| format!("{:02x}", self.rng.gen::<u8>()))
            .collect()
    }

    fn random_word(&mut self) -> String {
        let words = [
            "apple", "banana", "cherry", "data", "engine", "format",
            "graph", "hash", "index", "join", "kernel", "lambda",
            "matrix", "node", "object", "parse", "query", "route",
            "schema", "table", "union", "value", "widget", "xenon",
            "yield", "zone", "alpha", "beta", "gamma", "delta",
            "echo", "foxtrot", "golf", "hotel", "india", "juliet",
            "kilo", "lima", "mike", "november", "oscar", "papa",
            "quebec", "romeo", "sierra", "tango", "uniform", "victor",
            "whiskey", "xray", "yankee", "zulu", "red", "blue",
            "green", "orange", "purple", "silver", "golden", "dark",
            "light", "fast", "slow", "big", "small", "new", "old",
            "north", "south", "east", "west", "spring", "summer",
            "autumn", "winter", "sun", "moon", "star", "cloud",
        ];
        words[self.rng.gen_range(0..words.len())].to_string()
    }

    fn random_first_name(&mut self) -> String {
        let names = [
            "James", "Mary", "Robert", "Patricia", "John", "Jennifer",
            "Michael", "Linda", "David", "Elizabeth", "William", "Barbara",
            "Richard", "Susan", "Joseph", "Jessica", "Thomas", "Sarah",
            "Charles", "Karen", "Christopher", "Lisa", "Daniel", "Nancy",
            "Matthew", "Betty", "Anthony", "Margaret", "Mark", "Sandra",
            "Alexander", "Emily", "Benjamin", "Hannah", "Samuel", "Olivia",
            "Sophie", "Charlotte", "Amelia", "Mia", "Ava", "Grace",
            "Lucas", "Ethan", "Noah", "Oliver", "Liam", "Mason",
        ];
        names[self.rng.gen_range(0..names.len())].to_string()
    }

    fn random_last_name(&mut self) -> String {
        let names = [
            "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia",
            "Miller", "Davis", "Rodriguez", "Martinez", "Hernandez",
            "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas",
            "Taylor", "Moore", "Jackson", "Martin", "Lee", "Perez",
            "Thompson", "White", "Harris", "Sanchez", "Clark",
            "Ramirez", "Lewis", "Robinson", "Walker", "Young",
            "Allen", "King", "Wright", "Scott", "Torres", "Nguyen",
            "Hill", "Flores", "Green", "Adams", "Nelson", "Baker",
            "Hall", "Rivera", "Campbell", "Mitchell", "Carter",
        ];
        names[self.rng.gen_range(0..names.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_taxonomy() -> Taxonomy {
        Taxonomy::from_yaml(
            r#"
test.test.test:
  title: "Test"
  designation: universal
  locales: [UNIVERSAL]
  broad_type: VARCHAR
  release_priority: 1
"#,
        )
        .unwrap()
    }

    #[test]
    fn test_datetime_iso_8601() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("datetime.timestamp.iso_8601").unwrap();
        assert!(val.contains('T'));
        assert!(val.ends_with('Z'));
        assert_eq!(val.len(), 20);
    }

    #[test]
    fn test_datetime_date_us_slash() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("datetime.date.us_slash").unwrap();
        assert_eq!(val.len(), 10);
        assert!(val.contains('/'));
    }

    #[test]
    fn test_technology_ipv4() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("technology.internet.ip_v4").unwrap();
        assert_eq!(val.split('.').count(), 4);
    }

    #[test]
    fn test_technology_uuid() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("technology.cryptographic.uuid").unwrap();
        assert_eq!(val.len(), 36);
        assert_eq!(val.split('-').count(), 5);
    }

    #[test]
    fn test_identity_email() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("identity.person.email").unwrap();
        assert!(val.contains('@'));
        assert!(val.contains('.'));
    }

    #[test]
    fn test_identity_phone() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("identity.person.phone_number").unwrap();
        assert!(!val.is_empty());
    }

    #[test]
    fn test_geography_latitude() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("geography.coordinate.latitude").unwrap();
        let lat: f64 = val.parse().unwrap();
        assert!((-90.0..=90.0).contains(&lat));
    }

    #[test]
    fn test_representation_integer() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("representation.numeric.integer_number").unwrap();
        let _: i64 = val.parse().unwrap();
    }

    #[test]
    fn test_representation_hex_color() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("representation.text.color_hex").unwrap();
        assert!(val.len() == 7 || val.len() == 6);
    }

    #[test]
    fn test_container_json() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("container.object.json").unwrap();
        assert!(val.starts_with('{'));
        assert!(val.ends_with('}'));
    }

    #[test]
    fn test_container_query_string() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        let val = gen.generate_value("container.key_value.query_string").unwrap();
        assert!(val.contains('='));
        assert!(val.contains('&'));
    }

    #[test]
    fn test_all_domains_have_generators() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        // Test one type from each domain
        assert!(gen.generate_value("datetime.timestamp.iso_8601").is_ok());
        assert!(gen.generate_value("technology.internet.ip_v4").is_ok());
        assert!(gen.generate_value("identity.person.email").is_ok());
        assert!(gen.generate_value("geography.location.country").is_ok());
        assert!(gen.generate_value("representation.numeric.integer_number").is_ok());
        assert!(gen.generate_value("container.object.json").is_ok());
    }

    #[test]
    fn test_unknown_label_returns_error() {
        let mut gen = Generator::with_seed(test_taxonomy(), 42);
        assert!(gen.generate_value("nonexistent.type.foo").is_err());
    }
}
