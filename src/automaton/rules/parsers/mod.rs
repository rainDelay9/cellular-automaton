pub mod schemas;

use schemas::ConfigSchema;
pub use serde_json::Result;

pub fn parse_rules(data: &str) -> Result<ConfigSchema> {
    let schema: ConfigSchema = serde_json::from_str(data)?;
    Ok(schema)
}
