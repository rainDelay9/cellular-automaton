pub mod schemas;
pub mod types;

use schemas::ConfigSchema;
use serde::Deserialize;
pub use serde_json::Result;

pub fn parse_rules<'a, N>(data: &'a str) -> Result<ConfigSchema<N>>
where
    N: Deserialize<'a>,
{
    let schema: ConfigSchema<N> = serde_json::from_str(data)?;
    Ok(schema)
}
