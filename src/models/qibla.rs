use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct QiblaResponse {
    pub code: u16,
    pub status: String,
    pub data: QiblaData,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct QiblaData {
    pub latitude: f64,
    pub longitude: f64,
    pub direction: f64
}