use super::schemas::ConfigSchema;

pub type OneDimNeighborhood = Vec<u32>;
pub type TwoDimNeighborhood = Vec<Vec<u32>>;
pub type ThreeDimNeighborhood = Vec<Vec<Vec<u32>>>;

pub type OneDimRules = ConfigSchema<OneDimNeighborhood>;
pub type TwoDimRules = ConfigSchema<TwoDimNeighborhood>;
pub type ThreeDimRules = ConfigSchema<ThreeDimNeighborhood>;
