//! Synthetic data generation for training.
//!
//! Uses `fakeit` crate where available, custom generators for specialized formats.

use crate::taxonomy::Taxonomy;
use chrono::{NaiveDate, NaiveDateTime, Datelike};
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

    /// Generate a single value for a provider.method combination.
    fn generate_value(&mut self, provider: &str, method: &str) -> Result<String, GeneratorError> {
        match (provider, method) {
            // ═══════════════════════════════════════════════════════════════════
            // DATETIME - Custom generators for specific formats
            // ═══════════════════════════════════════════════════════════════════
            ("datetime", "iso_8601") => Ok(self.gen_iso_8601()),
            ("datetime", "iso_8601_compact") => Ok(self.gen_iso_8601_compact()),
            ("datetime", "iso_8601_ext") => Ok(self.gen_iso_8601_ext()),
            ("datetime", "iso_8601_with_time_zone_name") => Ok(self.gen_iso_8601_tz()),
            ("datetime", "rfc_2822") => Ok(self.gen_rfc_2822()),
            ("datetime", "rfc_2822_with_ordinals") => Ok(self.gen_rfc_2822_ordinal()),
            ("datetime", "rfc_3339") => Ok(self.gen_rfc_3339()),
            ("datetime", "unix_timestamp") => Ok(self.gen_unix_timestamp()),
            ("datetime", "unix_epoch_in_milliseconds") => Ok(self.gen_unix_millis()),
            ("datetime", "timestamp") => Ok(self.gen_unix_timestamp()),
            ("datetime", "sql_standard") => Ok(self.gen_sql_datetime()),
            ("datetime", "american") => Ok(self.gen_american_datetime()),
            ("datetime", "european") => Ok(self.gen_european_datetime()),
            ("datetime", "date") => Ok(self.gen_date_iso()),
            ("datetime", "datetime") => Ok(self.gen_datetime_iso()),
            ("datetime", "time") => Ok(self.gen_time_iso()),
            ("datetime", "duration") => Ok(self.gen_duration()),
            ("datetime", "timezone") => Ok(self.gen_timezone()),
            ("datetime", "gmt_offset") => Ok(self.gen_gmt_offset()),
            ("datetime", "century") => Ok(self.gen_century()),
            ("datetime", "periodicity") => Ok(self.gen_periodicity()),
            ("datetime", "day_of_month") => Ok(self.rng.gen_range(1..=31).to_string()),
            ("datetime", "day_of_week") => Ok(self.gen_day_of_week()),
            ("datetime", "year") => Ok(self.rng.gen_range(1990..2030).to_string()),
            ("datetime", "week_date") => Ok(self.gen_week_date()),
            ("datetime", "formatted_date") => Ok(self.gen_formatted_date()),
            ("datetime", "formatted_datetime") => Ok(self.gen_formatted_datetime()),
            ("datetime", "formatted_time") => Ok(self.gen_formatted_time()),
            ("datetime", "short_dmy") => Ok(self.gen_short_dmy()),
            ("datetime", "short_mdy") => Ok(self.gen_short_mdy()),
            ("datetime", "short_ymd") => Ok(self.gen_short_ymd()),
            
            // Date variants
            ("date", "abbreviated_month") => Ok(self.gen_abbreviated_month()),
            ("date", "full_weekday_abbreviated_month") => Ok(self.gen_full_weekday_abbr_month()),
            ("date", "long_full_month_name") => Ok(self.gen_long_full_month()),
            ("date", "long_weekday_month_name") => Ok(self.gen_long_weekday_month()),
            ("date", "julian") => Ok(self.gen_julian_date()),
            ("date", "ordinal") => Ok(self.gen_ordinal_date()),
            ("date", "month") => Ok(self.gen_month_name()),
            ("date", "numeric_dmy") => Ok(self.gen_numeric_dmy()),
            ("date", "numeric_mdy") => Ok(self.gen_numeric_mdy()),
            ("date", "numeric_ymd") => Ok(self.gen_numeric_ymd()),

            // ═══════════════════════════════════════════════════════════════════
            // INTERNET - Mix of fakeit and custom
            // ═══════════════════════════════════════════════════════════════════
            ("internet", "ip_v4") => Ok(fakeit::internet::ipv4_address()),
            ("internet", "ip_v4_with_port") => Ok(format!("{}:{}", fakeit::internet::ipv4_address(), self.rng.gen_range(1024..65535))),
            ("internet", "ip_v6") => Ok(fakeit::internet::ipv6_address()),
            ("internet", "mac_address") => Ok(fakeit::internet::mac_address()),
            ("internet", "url") => Ok(format!("https://{}/", fakeit::internet::domain_name())),
            ("internet", "uri") => Ok(format!("https://{}/{}", fakeit::internet::domain_name(), fakeit::words::word())),
            ("internet", "hostname") => Ok(fakeit::internet::domain_name()),
            ("internet", "top_level_domain") => Ok(format!(".{}", fakeit::internet::domain_suffix())),
            ("internet", "tld") => Ok(format!(".{}", fakeit::internet::domain_suffix())),
            ("internet", "slug") => Ok(self.gen_slug()),
            ("internet", "query_string") => Ok(self.gen_query_string()),
            ("internet", "user_agent") => Ok(fakeit::user_agent::random_platform()),
            ("internet", "http_method") => Ok(fakeit::internet::http_method()),
            ("internet", "http_status_code") => Ok(fakeit::status_code::simple().to_string()),
            ("internet", "http_status_message") => Ok(format!("{} {}", fakeit::status_code::simple(), self.gen_status_text())),
            ("internet", "public_dns") => Ok(self.gen_public_dns()),
            ("internet", "asn") => Ok(format!("AS{}", self.rng.gen_range(1000..4000000000u64))),
            ("internet", "port") => Ok(self.rng.gen_range(1..65535).to_string()),
            ("internet", "dsn") => Ok(self.gen_dsn()),
            ("internet", "path") => Ok(self.gen_url_path()),
            ("internet", "content_type") => Ok(self.gen_content_type()),

            // ═══════════════════════════════════════════════════════════════════
            // CRYPTOGRAPHIC
            // ═══════════════════════════════════════════════════════════════════
            ("cryptographic", "uuid") => Ok(Uuid::new_v4().to_string()),
            ("cryptographic", "hash") => Ok(self.gen_hash(32)),
            ("cryptographic", "token_hex") => Ok(self.gen_hash(32)),
            ("cryptographic", "token_urlsafe") => Ok(self.gen_token_urlsafe()),
            ("cryptographic", "mnemonic_phrase") => Ok(self.gen_mnemonic_phrase()),

            // ═══════════════════════════════════════════════════════════════════
            // PERSON - Using fakeit
            // ═══════════════════════════════════════════════════════════════════
            ("person", "email") => Ok(fakeit::contact::email()),
            ("person", "first_name") => Ok(fakeit::name::first()),
            ("person", "last_name") => Ok(fakeit::name::last()),
            ("person", "full_name") => Ok(fakeit::name::full()),
            ("person", "phone_number") => Ok(fakeit::contact::phone_formatted()),
            ("person", "username") => Ok(fakeit::internet::username()),
            ("person", "password") => Ok(fakeit::password::generate(true, true, true, 12)),
            ("person", "gender") => Ok(fakeit::person::gender()),
            ("person", "gender_code") => Ok(self.rng.gen_range(0..3).to_string()),
            ("person", "gender_symbol") => Ok(["♂", "♀", "⚲"][self.rng.gen_range(0..3)].to_string()),
            ("person", "title") => Ok(fakeit::name::prefix()),
            ("person", "name") => Ok(fakeit::name::first()),
            ("person", "surname") => Ok(fakeit::name::last()),
            ("person", "telephone") => Ok(fakeit::contact::phone_formatted()),
            ("person", "academic_degree") => Ok(["Bachelor", "Master", "Doctorate", "PhD"][self.rng.gen_range(0..4)].to_string()),
            ("person", "language") => Ok(fakeit::language::random()),
            ("person", "nationality") => Ok(fakeit::address::country()),
            ("person", "occupation") => Ok(fakeit::job::title()),
            ("person", "university") => Ok(format!("{} University", fakeit::address::city())),
            ("person", "political_views") => Ok(["Liberal", "Conservative", "Moderate", "Socialist"][self.rng.gen_range(0..4)].to_string()),
            ("person", "worldview") => Ok(["Atheism", "Christianity", "Islam", "Buddhism", "Hinduism", "Judaism"][self.rng.gen_range(0..6)].to_string()),
            ("person", "views_on") => Ok(["Positive", "Negative", "Neutral", "Compromisable"][self.rng.gen_range(0..4)].to_string()),
            ("person", "blood_type") => Ok(["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"][self.rng.gen_range(0..8)].to_string()),
            ("person", "sex") => Ok(["Male", "Female", "Other"][self.rng.gen_range(0..3)].to_string()),
            ("person", "height") => Ok(format!("{:.2}", self.rng.gen_range(150..200) as f32 / 100.0)),
            ("person", "weight") => Ok(self.rng.gen_range(45..120).to_string()),
            ("person", "identifier") => Ok(format!("{:02}-{:02}/{:02}", self.rng.gen_range(10..99), self.rng.gen_range(10..99), self.rng.gen_range(10..99))),

            // ═══════════════════════════════════════════════════════════════════
            // ADDRESS - Using fakeit
            // ═══════════════════════════════════════════════════════════════════
            ("address", "address") => Ok(fakeit::address::street()),
            ("address", "city") => Ok(fakeit::address::city()),
            ("address", "country") => Ok(fakeit::address::country()),
            ("address", "country_code") => Ok(fakeit::address::country_abr()),
            ("address", "country_emoji_flag") => Ok(self.gen_country_flag()),
            ("address", "postal_code") => Ok(fakeit::address::zip()),
            ("address", "zip_code") => Ok(fakeit::address::zip()),
            ("address", "state") => Ok(fakeit::address::state()),
            ("address", "street_name") => Ok(fakeit::address::street_name()),
            ("address", "street_number") => Ok(fakeit::address::street_number()),
            ("address", "street_suffix") => Ok(fakeit::address::street_suffix()),
            ("address", "latitude") => Ok(fakeit::address::latitude().to_string()),
            ("address", "longitude") => Ok(fakeit::address::longitude().to_string()),
            ("address", "iata_code") => Ok(self.gen_iata_code()),
            ("address", "icao_code") => Ok(self.gen_icao_code()),
            ("address", "isd_code") => Ok(format!("+{}", self.rng.gen_range(1..999))),
            ("address", "calling_code") => Ok(format!("+{}", self.rng.gen_range(1..999))),
            ("address", "continent") => Ok(["Africa", "Antarctica", "Asia", "Europe", "North America", "Oceania", "South America"][self.rng.gen_range(0..7)].to_string()),
            ("address", "region") => Ok(fakeit::address::state()),
            ("address", "province") => Ok(fakeit::address::state()),
            ("address", "prefecture") => Ok(fakeit::address::state()),
            ("address", "federal_subject") => Ok(fakeit::address::state()),

            // ═══════════════════════════════════════════════════════════════════
            // CODE - Identifiers
            // ═══════════════════════════════════════════════════════════════════
            ("code", "ean") => Ok(self.gen_ean()),
            ("code", "imei") => Ok(self.gen_imei()),
            ("code", "isbn") => Ok(self.gen_isbn()),
            ("code", "issn") => Ok(self.gen_issn()),
            ("code", "locale_code") => Ok(self.gen_locale_code()),
            ("code", "pin") => Ok(format!("{:04}", self.rng.gen_range(0..10000))),

            // ═══════════════════════════════════════════════════════════════════
            // PAYMENT
            // ═══════════════════════════════════════════════════════════════════
            ("payment", "bitcoin_address") => Ok(self.gen_bitcoin_address()),
            ("payment", "ethereum_address") => Ok(self.gen_ethereum_address()),
            ("payment", "credit_card_number") => Ok(fakeit::payment::credit_card_number()),
            ("payment", "credit_card_expiration_date") => Ok(fakeit::payment::credit_card_exp()),
            ("payment", "credit_card_network") => Ok(fakeit::payment::credit_card_type()),
            ("payment", "cvv") => Ok(fakeit::payment::credit_card_cvv()),
            ("payment", "cid") => Ok(format!("{:04}", self.rng.gen_range(0..10000))),
            ("payment", "paypal") => Ok(fakeit::contact::email()),

            // ═══════════════════════════════════════════════════════════════════
            // FINANCE
            // ═══════════════════════════════════════════════════════════════════
            ("finance", "currency_iso_code") => Ok(fakeit::currency::short()),
            ("finance", "currency_symbol") => Ok(["$", "€", "£", "¥", "₹", "₽"][self.rng.gen_range(0..6)].to_string()),
            ("finance", "cryptocurrency_iso_code") => Ok(["BTC", "ETH", "XRP", "LTC", "BCH", "ADA", "DOT", "LINK", "BNB", "USDT"][self.rng.gen_range(0..10)].to_string()),
            ("finance", "cryptocurrency_symbol") => Ok(["₿", "Ξ", "Ł"][self.rng.gen_range(0..3)].to_string()),
            ("finance", "stock_ticker") => Ok(self.gen_stock_ticker()),
            ("finance", "stock_name") => Ok(fakeit::company::company()),
            ("finance", "stock_exchange") => Ok(["NYSE", "NASDAQ", "LSE", "TSE", "SSE", "HKEX"][self.rng.gen_range(0..6)].to_string()),
            ("finance", "company") => Ok(fakeit::company::company()),
            ("finance", "company_type") => Ok(fakeit::company::company_suffix()),
            ("finance", "bank") => Ok(format!("{} Bank", fakeit::company::company())),
            ("finance", "price") => Ok(format!("{:.2}", self.rng.gen_range(1..10000) as f32 + self.rng.gen::<f32>())),
            ("finance", "price_in_btc") => Ok(format!("{:.7}", self.rng.gen::<f32>() * 2.0)),

            // ═══════════════════════════════════════════════════════════════════
            // SCIENCE
            // ═══════════════════════════════════════════════════════════════════
            ("science", "dna_sequence") => Ok(self.gen_dna_sequence()),
            ("science", "rna_sequence") => Ok(self.gen_rna_sequence()),
            ("science", "measure_unit") => Ok(["meter", "gram", "second", "ampere", "kelvin", "mole", "candela"][self.rng.gen_range(0..7)].to_string()),
            ("science", "metric_prefix") => Ok(["kilo", "mega", "giga", "milli", "micro", "nano", "pico"][self.rng.gen_range(0..7)].to_string()),

            // ═══════════════════════════════════════════════════════════════════
            // DEVELOPMENT
            // ═══════════════════════════════════════════════════════════════════
            ("development", "version") => Ok(fakeit::unique::uuid_v4().split('-').next().unwrap_or("1.0.0").to_string().replace(|c: char| !c.is_numeric(), ".")),
            ("development", "boolean") => Ok(fakeit::bool_rand::bool().to_string()),
            ("development", "calver") => Ok(format!("{}.{}.{}", self.rng.gen_range(2018..2026), self.rng.gen_range(1..13), self.rng.gen_range(1..29))),
            ("development", "programming_language") => Ok(fakeit::hacker::noun()),
            ("development", "os") => Ok(["Windows 10", "Windows 11", "macOS", "Ubuntu", "Fedora", "Debian"][self.rng.gen_range(0..6)].to_string()),
            ("development", "software_license") => Ok(["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause", "ISC", "MPL-2.0"][self.rng.gen_range(0..6)].to_string()),
            ("development", "stage") => Ok(["Alpha", "Beta", "RC", "Stable", "LTS"][self.rng.gen_range(0..5)].to_string()),
            ("development", "ility") => Ok(["scalability", "reliability", "maintainability", "usability", "security"][self.rng.gen_range(0..5)].to_string()),
            ("development", "system_quality_attribute") => Ok(["performance", "security", "reliability", "scalability"][self.rng.gen_range(0..4)].to_string()),

            // ═══════════════════════════════════════════════════════════════════
            // FILE
            // ═══════════════════════════════════════════════════════════════════
            ("file", "extension") => Ok(fakeit::file::extension()),
            ("file", "file_name") => Ok(format!("{}.{}", fakeit::words::word(), fakeit::file::extension())),
            ("file", "mime_type") => Ok(fakeit::file::mime_type()),
            ("file", "size") => Ok(format!("{} {}", self.rng.gen_range(1..1000), ["bytes", "KB", "MB", "GB"][self.rng.gen_range(0..4)])),

            // ═══════════════════════════════════════════════════════════════════
            // TEXT
            // ═══════════════════════════════════════════════════════════════════
            ("text", "word") => Ok(fakeit::words::word()),
            ("text", "sentence") => Ok(fakeit::words::sentence(8)),
            ("text", "text") => Ok(fakeit::words::paragraph(3, 4, 8, " ".to_string())),
            ("text", "title") => Ok(fakeit::words::sentence(6)),
            ("text", "color") => Ok(fakeit::color::full()),
            ("text", "hex_color") => Ok(fakeit::color::hex()),
            ("text", "rgb_color") => Ok(format!("rgb({}, {}, {})", self.rng.gen_range(0..256), self.rng.gen_range(0..256), self.rng.gen_range(0..256))),
            ("text", "emoji") => Ok(fakeit::unique::uuid_v4().chars().next().unwrap_or('😀').to_string()), // placeholder
            ("text", "quote") => Ok(fakeit::hipster::sentence(10)),
            ("text", "answer") => Ok(["Yes", "No", "Maybe"][self.rng.gen_range(0..3)].to_string()),
            ("text", "level") => Ok(["low", "medium", "high", "critical"][self.rng.gen_range(0..4)].to_string()),

            // ═══════════════════════════════════════════════════════════════════
            // NUMERIC
            // ═══════════════════════════════════════════════════════════════════
            ("numeric", "integer_number") => Ok(self.rng.gen_range(-1000..1000).to_string()),
            ("numeric", "float_number") => Ok(format!("{:.6}", (self.rng.gen::<f64>() - 0.5) * 2000.0)),
            ("numeric", "decimal_number") => Ok(format!("{:.20}", (self.rng.gen::<f64>() - 0.5) * 2000.0)),
            ("numeric", "complex_number") => Ok(format!("{}+{}j", self.rng.gen::<f64>(), self.rng.gen::<f64>())),
            ("numeric", "increment") => Ok(self.rng.gen_range(1..1000).to_string()),

            // ═══════════════════════════════════════════════════════════════════
            // HARDWARE
            // ═══════════════════════════════════════════════════════════════════
            ("hardware", "cpu") => Ok(["Intel Core i9", "Intel Core i7", "AMD Ryzen 9", "AMD Ryzen 7", "Apple M1", "Apple M2"][self.rng.gen_range(0..6)].to_string()),
            ("hardware", "cpu_codename") => Ok(["Alder Lake", "Raptor Lake", "Zen 4", "Ice Lake"][self.rng.gen_range(0..4)].to_string()),
            ("hardware", "cpu_frequency") => Ok(format!("{}.{}GHz", self.rng.gen_range(2..5), self.rng.gen_range(0..9))),
            ("hardware", "graphics") => Ok(["NVIDIA RTX 4090", "NVIDIA RTX 3080", "AMD RX 7900", "Intel Arc A770"][self.rng.gen_range(0..4)].to_string()),
            ("hardware", "manufacturer") => Ok(["Apple", "Dell", "HP", "Lenovo", "ASUS", "Acer"][self.rng.gen_range(0..6)].to_string()),
            ("hardware", "phone_model") => Ok(["iPhone 15 Pro", "Samsung Galaxy S24", "Google Pixel 8", "OnePlus 12"][self.rng.gen_range(0..4)].to_string()),
            ("hardware", "ram_size") => Ok(["8GB", "16GB", "32GB", "64GB", "128GB"][self.rng.gen_range(0..5)].to_string()),
            ("hardware", "ram_type") => Ok(["DDR4", "DDR5", "LPDDR5", "GDDR6"][self.rng.gen_range(0..4)].to_string()),
            ("hardware", "resolution") => Ok(["1920x1080", "2560x1440", "3840x2160", "5120x2880"][self.rng.gen_range(0..4)].to_string()),
            ("hardware", "screen_size") => Ok(["13\"", "14\"", "15\"", "16\"", "27\"", "32\""][self.rng.gen_range(0..6)].to_string()),
            ("hardware", "ssd_or_hdd") => Ok(format!("{} {} {}", ["Samsung", "WD", "Seagate"][self.rng.gen_range(0..3)], ["256GB", "512GB", "1TB", "2TB"][self.rng.gen_range(0..4)], ["SSD", "HDD"][self.rng.gen_range(0..2)])),
            ("hardware", "generation") => Ok(format!("{} Generation", ["1st", "2nd", "3rd", "4th", "5th"][self.rng.gen_range(0..5)])),

            // ═══════════════════════════════════════════════════════════════════
            // FOOD
            // ═══════════════════════════════════════════════════════════════════
            ("food", "dish") => Ok(fakeit::hipster::word()),
            ("food", "drink") => Ok(fakeit::beer::name()),
            ("food", "fruit") => Ok(["Apple", "Banana", "Orange", "Mango", "Grape", "Strawberry"][self.rng.gen_range(0..6)].to_string()),
            ("food", "vegetable") => Ok(["Carrot", "Broccoli", "Spinach", "Tomato", "Potato", "Onion"][self.rng.gen_range(0..6)].to_string()),
            ("food", "spices") => Ok(["Pepper", "Salt", "Cumin", "Cinnamon", "Oregano", "Basil"][self.rng.gen_range(0..6)].to_string()),

            // ═══════════════════════════════════════════════════════════════════
            // TRANSPORT
            // ═══════════════════════════════════════════════════════════════════
            ("transport", "airplane") => Ok(["Boeing 737", "Boeing 787", "Airbus A320", "Airbus A380"][self.rng.gen_range(0..4)].to_string()),
            ("transport", "car") => Ok(format!("{} {}", fakeit::vehicle::car_maker(), fakeit::vehicle::car_model())),
            ("transport", "manufacturer") => Ok(fakeit::vehicle::car_maker()),
            ("transport", "vehicle_registration_code") => Ok(fakeit::address::country_abr()),

            // ═══════════════════════════════════════════════════════════════════
            // PATH
            // ═══════════════════════════════════════════════════════════════════
            ("path", "home") => Ok("/home".to_string()),
            ("path", "root") => Ok("/".to_string()),
            ("path", "user") => Ok(format!("/home/{}", fakeit::internet::username())),
            ("path", "dev_dir") => Ok(format!("/home/{}/dev/{}", fakeit::internet::username(), ["Go", "Rust", "Python"][self.rng.gen_range(0..3)])),
            ("path", "project_dir") => Ok(format!("/home/{}/dev/{}/{}", fakeit::internet::username(), ["Go", "Rust", "Python"][self.rng.gen_range(0..3)], fakeit::words::word())),
            ("path", "users_folder") => Ok(format!("/home/{}/{}", fakeit::internet::username(), ["Documents", "Downloads", "Desktop"][self.rng.gen_range(0..3)])),

            _ => Err(GeneratorError::NotImplemented(format!("{}.{}", provider, method))),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom datetime generators
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

    fn gen_iso_8601(&mut self) -> String {
        self.random_datetime().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    fn gen_iso_8601_compact(&mut self) -> String {
        self.random_datetime().format("%Y%m%dT%H%M%S").to_string()
    }

    fn gen_iso_8601_ext(&mut self) -> String {
        let dt = self.random_datetime();
        let micros = self.rng.gen_range(0..1000000);
        format!("{}.{:06}Z", dt.format("%Y-%m-%dT%H:%M:%S"), micros)
    }

    fn gen_iso_8601_tz(&mut self) -> String {
        let dt = self.random_datetime();
        let offset = self.rng.gen_range(-12..=12);
        format!("{}{}{}:00", dt.format("%Y-%m-%dT%H:%M:%S"), if offset >= 0 { "+" } else { "" }, offset)
    }

    fn gen_rfc_2822(&mut self) -> String {
        self.random_datetime().format("%a, %d %b %Y %H:%M:%S GMT+00:00").to_string()
    }

    fn gen_rfc_2822_ordinal(&mut self) -> String {
        let dt = self.random_datetime();
        let day = dt.day();
        let ordinal = match day {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        };
        format!("{}, {:02}`{}` {} +0000",
            dt.format("%a"), day, ordinal, dt.format("%b %Y %H:%M:%S"))
    }

    fn gen_rfc_3339(&mut self) -> String {
        format!("{} GMT+00:00", self.random_datetime().format("%Y-%m-%dT%H:%M:%S"))
    }

    fn gen_unix_timestamp(&mut self) -> String {
        self.rng.gen_range(1_000_000_000i64..2_000_000_000).to_string()
    }

    fn gen_unix_millis(&mut self) -> String {
        self.rng.gen_range(1_000_000_000_000i64..2_000_000_000_000).to_string()
    }

    fn gen_sql_datetime(&mut self) -> String {
        self.random_datetime().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn gen_american_datetime(&mut self) -> String {
        self.random_datetime().format("%m/%d/%Y %I:%M %p").to_string()
    }

    fn gen_european_datetime(&mut self) -> String {
        self.random_datetime().format("%d/%m/%Y %H:%M").to_string()
    }

    fn gen_date_iso(&mut self) -> String {
        self.random_datetime().format("%Y-%m-%d").to_string()
    }

    fn gen_datetime_iso(&mut self) -> String {
        let dt = self.random_datetime();
        let micros = self.rng.gen_range(0..1000000);
        format!("{}.{:06}", dt.format("%Y-%m-%dT%H:%M:%S"), micros)
    }

    fn gen_time_iso(&mut self) -> String {
        let dt = self.random_datetime();
        let micros = self.rng.gen_range(0..1000000);
        format!("{}.{:06}", dt.format("%H:%M:%S"), micros)
    }

    fn gen_duration(&mut self) -> String {
        format!("PT{}M", self.rng.gen_range(1..60))
    }

    fn gen_timezone(&mut self) -> String {
        ["America/New_York", "Europe/London", "Asia/Tokyo", "Australia/Sydney", 
         "Pacific/Auckland", "America/Los_Angeles", "Europe/Paris", "Asia/Shanghai",
         "America/Chicago", "Europe/Berlin", "Asia/Singapore", "Africa/Cairo"][self.rng.gen_range(0..12)].to_string()
    }

    fn gen_gmt_offset(&mut self) -> String {
        let offset = self.rng.gen_range(-12i32..=14);
        format!("UTC {:+03}:00", offset)
    }

    fn gen_century(&mut self) -> String {
        ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", 
         "XI", "XII", "XIII", "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX", "XXI"][self.rng.gen_range(0..21)].to_string()
    }

    fn gen_periodicity(&mut self) -> String {
        ["Once", "Daily", "Weekly", "Monthly", "Quarterly", "Yearly", "Never"][self.rng.gen_range(0..7)].to_string()
    }

    fn gen_day_of_week(&mut self) -> String {
        ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"][self.rng.gen_range(0..7)].to_string()
    }

    fn gen_week_date(&mut self) -> String {
        format!("{}-W{}", self.rng.gen_range(2015..2030), self.rng.gen_range(1..53))
    }

    fn gen_formatted_date(&mut self) -> String {
        self.random_datetime().format("%m/%d/%Y").to_string()
    }

    fn gen_formatted_datetime(&mut self) -> String {
        self.random_datetime().format("%m/%d/%Y %H:%M:%S").to_string()
    }

    fn gen_formatted_time(&mut self) -> String {
        self.random_datetime().format("%H:%M:%S").to_string()
    }

    fn gen_short_dmy(&mut self) -> String {
        self.random_datetime().format("%d-%m-%y").to_string()
    }

    fn gen_short_mdy(&mut self) -> String {
        self.random_datetime().format("%m-%d-%y").to_string()
    }

    fn gen_short_ymd(&mut self) -> String {
        self.random_datetime().format("%y-%m-%d").to_string()
    }

    fn gen_abbreviated_month(&mut self) -> String {
        self.random_datetime().format("%b %d, %Y").to_string()
    }

    fn gen_full_weekday_abbr_month(&mut self) -> String {
        self.random_datetime().format("%A, %d %b %Y").to_string()
    }

    fn gen_long_full_month(&mut self) -> String {
        self.random_datetime().format("%B %d, %Y").to_string()
    }

    fn gen_long_weekday_month(&mut self) -> String {
        self.random_datetime().format("%A, %d %B %Y").to_string()
    }

    fn gen_julian_date(&mut self) -> String {
        format!("{:02}-{:03}", self.rng.gen_range(20..30), self.rng.gen_range(1..366))
    }

    fn gen_ordinal_date(&mut self) -> String {
        format!("{}-{:03}", self.rng.gen_range(2020..2030), self.rng.gen_range(1..366))
    }

    fn gen_month_name(&mut self) -> String {
        ["January", "February", "March", "April", "May", "June", 
         "July", "August", "September", "October", "November", "December"][self.rng.gen_range(0..12)].to_string()
    }

    fn gen_numeric_dmy(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{:02}{:02}{}", dt.day(), dt.month(), dt.year())
    }

    fn gen_numeric_mdy(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{:02}{:02}{}", dt.month(), dt.day(), dt.year())
    }

    fn gen_numeric_ymd(&mut self) -> String {
        let dt = self.random_datetime();
        format!("{}{:02}{:02}", dt.year(), dt.month(), dt.day())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom internet generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_slug(&mut self) -> String {
        let count = self.rng.gen_range(3..8);
        (0..count)
            .map(|_| fakeit::words::word())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn gen_query_string(&mut self) -> String {
        let count = self.rng.gen_range(1..4);
        (0..count)
            .map(|_| format!("{}={}", fakeit::words::word(), fakeit::words::word()))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn gen_public_dns(&mut self) -> String {
        ["8.8.8.8", "8.8.4.4", "1.1.1.1", "1.0.0.1", "208.67.222.222", "208.67.220.220", "9.9.9.9"][self.rng.gen_range(0..7)].to_string()
    }

    fn gen_dsn(&mut self) -> String {
        let protos = ["postgres", "mysql", "mongodb", "redis", "memcached"];
        let port = [5432, 3306, 27017, 6379, 11211][self.rng.gen_range(0..5)];
        format!("{}://{}:{}", protos[self.rng.gen_range(0..5)], fakeit::internet::domain_name(), port)
    }

    fn gen_url_path(&mut self) -> String {
        let depth = self.rng.gen_range(2..6);
        (0..depth)
            .map(|_| fakeit::words::word())
            .collect::<Vec<_>>()
            .join("/")
    }

    fn gen_content_type(&mut self) -> String {
        ["text/plain", "text/html", "text/css", "application/json", "application/xml",
         "application/javascript", "image/png", "image/jpeg", "image/gif", "audio/mpeg",
         "video/mp4", "application/pdf", "application/octet-stream"][self.rng.gen_range(0..13)].to_string()
    }

    fn gen_status_text(&mut self) -> String {
        ["OK", "Created", "Not Found", "Internal Server Error", "Bad Request", 
         "Unauthorized", "Forbidden", "Service Unavailable"][self.rng.gen_range(0..8)].to_string()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom crypto generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_hash(&mut self, bytes: usize) -> String {
        (0..bytes)
            .map(|_| format!("{:02x}", self.rng.gen::<u8>()))
            .collect()
    }

    fn gen_token_urlsafe(&mut self) -> String {
        use rand::distributions::Alphanumeric;
        (0..43)
            .map(|_| self.rng.sample(Alphanumeric) as char)
            .collect::<String>()
            .replace('+', "-")
            .replace('/', "_")
    }

    fn gen_mnemonic_phrase(&mut self) -> String {
        (0..12)
            .map(|_| fakeit::words::word())
            .collect::<Vec<_>>()
            .join(" ")
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom address generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_country_flag(&mut self) -> String {
        ["🇺🇸", "🇬🇧", "🇨🇦", "🇦🇺", "🇩🇪", "🇫🇷", "🇯🇵", "🇧🇷", "🇮🇳", "🇨🇳"][self.rng.gen_range(0..10)].to_string()
    }

    fn gen_iata_code(&mut self) -> String {
        (0..3).map(|_| (b'A' + self.rng.gen_range(0..26)) as char).collect()
    }

    fn gen_icao_code(&mut self) -> String {
        (0..4).map(|_| (b'A' + self.rng.gen_range(0..26)) as char).collect()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom code generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_ean(&mut self) -> String {
        if self.rng.gen_bool(0.5) {
            format!("{:08}", self.rng.gen_range(10000000u64..99999999))
        } else {
            format!("{:013}", self.rng.gen_range(1000000000000u64..9999999999999))
        }
    }

    fn gen_imei(&mut self) -> String {
        format!("{:015}", self.rng.gen_range(100000000000000u64..999999999999999))
    }

    fn gen_isbn(&mut self) -> String {
        format!("{}-{}-{:05}-{:03}-{}", 
            self.rng.gen_range(1..9),
            self.rng.gen_range(1..9),
            self.rng.gen_range(10000..99999),
            self.rng.gen_range(100..999),
            self.rng.gen_range(0..9))
    }

    fn gen_issn(&mut self) -> String {
        format!("{:04}-{:04}", self.rng.gen_range(1000..9999), self.rng.gen_range(1000..9999))
    }

    fn gen_locale_code(&mut self) -> String {
        ["en-us", "en-gb", "de-de", "fr-fr", "es-es", "ja-jp", "zh-cn", "pt-br", "ru-ru", "ko-kr"][self.rng.gen_range(0..10)].to_string()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom payment generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_bitcoin_address(&mut self) -> String {
        let prefix = ["1", "3", "bc1"][self.rng.gen_range(0..3)];
        let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        let chars: String = (0..33)
            .map(|_| base58_chars.chars().nth(self.rng.gen_range(0..58)).unwrap())
            .collect();
        format!("{}{}", prefix, chars)
    }

    fn gen_ethereum_address(&mut self) -> String {
        format!("0x{}", self.gen_hash(20))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom finance generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_stock_ticker(&mut self) -> String {
        let len = self.rng.gen_range(2..5);
        (0..len).map(|_| (b'A' + self.rng.gen_range(0..26)) as char).collect()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom science generators
    // ═══════════════════════════════════════════════════════════════════════════

    fn gen_dna_sequence(&mut self) -> String {
        (0..10).map(|_| ['A', 'C', 'G', 'T'][self.rng.gen_range(0..4)]).collect()
    }

    fn gen_rna_sequence(&mut self) -> String {
        (0..10).map(|_| ['A', 'C', 'G', 'U'][self.rng.gen_range(0..4)]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_taxonomy() -> Taxonomy {
        Taxonomy::from_yaml("test.label:\n  provider: test\n  method: label\n  release_priority: 1\n  locales: [UNIVERSAL]").unwrap()
    }

    #[test]
    fn test_ipv4_generation() {
        let mut gen = Generator::with_seed(empty_taxonomy(), 42);
        let ip = gen.generate_value("internet", "ip_v4").unwrap();
        assert!(ip.split('.').count() == 4);
    }

    #[test]
    fn test_uuid_generation() {
        let mut gen = Generator::with_seed(empty_taxonomy(), 42);
        let uuid = gen.generate_value("cryptographic", "uuid").unwrap();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_email_generation() {
        let mut gen = Generator::with_seed(empty_taxonomy(), 42);
        let email = gen.generate_value("person", "email").unwrap();
        assert!(email.contains('@'));
    }

    #[test]
    fn test_iso_8601() {
        let mut gen = Generator::with_seed(empty_taxonomy(), 42);
        let dt = gen.generate_value("datetime", "iso_8601").unwrap();
        assert!(dt.contains('T'));
        assert!(dt.ends_with('Z'));
    }
}
