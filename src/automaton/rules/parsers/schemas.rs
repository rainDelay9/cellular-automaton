use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleSchema {
    pub neighborhood: Vec<u32>,
    pub cell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesSchema {
    pub rules: Vec<RuleSchema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoordinatesSchema {
    pub coordinates: Vec<Vec<usize>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DimensionsSchema {
    pub dimensions: Vec<usize>,
}
