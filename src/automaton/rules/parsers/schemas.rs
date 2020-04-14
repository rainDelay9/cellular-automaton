use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RulesSchema<N> {
    pub neighborhood: N,
    pub cell: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSchema<N> {
    pub dimensions: Vec<u32>,
    pub rules: Vec<RulesSchema<N>>,
}
