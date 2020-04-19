use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExplicitRuleSchema {
    pub neighborhood: Vec<u32>,
    pub current: u32,
    pub next: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesSchema {
    pub explicit_rules: Vec<ExplicitRuleSchema>,
    pub sum_rules: Vec<SumRuleSchema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SumRuleSchema {
    pub rule_type: u32,
    pub neighborhood: u32,
    pub current: u32,
    pub next: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SumRulesSchema {
    pub rules: Vec<SumRuleSchema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoordinatesSchema {
    pub coordinates: Vec<Vec<usize>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DimensionsSchema {
    pub dimensions: Vec<usize>,
}
