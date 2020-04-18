pub mod schemas;

use schemas::{ConfigSchema, CoordinatesSchema};
pub use serde_json::Result;

pub fn parse_rules(data: &str) -> Result<ConfigSchema> {
    let schema: ConfigSchema = serde_json::from_str(data)?;
    Ok(schema)
}

pub fn parse_coordinates(data: &str) -> Result<CoordinatesSchema> {
    let schema: CoordinatesSchema = serde_json::from_str(data)?;
    Ok(schema)
}
