use serde::{Deserialize, Serialize};

use crate::lights::PointLight;
use crate::lights::SpotLight;

#[derive(Debug, Serialize, Deserialize)]
pub enum LightsConfig {
    Point(PointLight),
    Spot(SpotLight),
}
