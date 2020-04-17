use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesSchema {
    pub neighborhood: Vec<u32>,
    pub cell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigSchema {
    pub dimensions: Vec<usize>,
    pub rules: Vec<RulesSchema>,
}
