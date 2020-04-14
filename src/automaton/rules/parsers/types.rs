use super::schemas::ConfigSchema;

pub type OneDimention = Vec<u32>;
pub type TwoDimDimentions = Vec<Vec<u32>>;
pub type ThreeDimDimentions = Vec<Vec<Vec<u32>>>;

pub type OneDimRules = ConfigSchema<OneDimention>;
pub type TwoDimRules = ConfigSchema<TwoDimDimentions>;
pub type ThreeDimRules = ConfigSchema<ThreeDimDimentions>;
