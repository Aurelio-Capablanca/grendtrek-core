use serde::Deserialize;

use crate::internals::data_structures::database_connector_spec::VendorOptions;

#[derive(Debug, Deserialize)]
pub struct PGCollation {
    provider: String,
    locale: String,
    deterministic: bool,
    description: String,
}

impl PGCollation {
    
    pub fn get_provider_as_ref(&self) -> &str {
        &self.provider
    }
    pub fn get_locale_as_ref(&self) -> &str {
        &self.locale
    }

    pub fn get_deterministic_as_ref(&self) -> &bool {
        &self.deterministic
    }

    pub fn get_descrip_as_ref(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Deserialize)]
pub enum DBCollation {
    MSSQL(String),
    POSTGRES(PGCollation),
}

#[derive(Debug, Deserialize)]
pub struct Collations {
    engine_origin: VendorOptions,
    engine_destiny: VendorOptions,
    collation_origin: DBCollation,
    collation_destiny: DBCollation,
}

impl Collations {
    pub fn get_origin_engine_ref(&self) -> &VendorOptions {
        &self.engine_origin
    }

    pub fn get_destiny_engine_ref(&self) -> &VendorOptions {
        &self.engine_destiny
    }

    pub fn get_collation_origin_ref(&self) -> &DBCollation {
        &self.collation_origin
    }

    pub fn get_collation_destiny_ref(&self) -> &DBCollation {
        &self.collation_destiny
    }
}
