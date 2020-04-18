pub mod schemas;

use exitfailure::ExitFailure;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;

pub fn parse_file_to_schema<T: DeserializeOwned>(path: &PathBuf) -> Result<T, ExitFailure> {
    let data = fs::read_to_string(path)?;
    let schema: T = serde_json::from_str(&data)?;
    Ok(schema)
}
