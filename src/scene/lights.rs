use serde::{Deserialize, Serialize};

use crate::lights::{AreaLightConfig, PointLight, SpotLight};

#[derive(Debug, Serialize, Deserialize)]
pub enum LightsConfig {
    Point(PointLight),
    Spot(SpotLight),
    Area(AreaLightConfig),
}
