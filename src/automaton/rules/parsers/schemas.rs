use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RulesSchema {
    pub neighborhood: Vec<u32>,
    pub cell: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSchema {
    pub dimensions: Vec<u32>,
    pub rules: Vec<RulesSchema>,
}
