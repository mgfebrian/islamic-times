use serde::{Deserialize, Serialize};

// 1. Struct untuk jam sholat
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct PrayerTimings {
    #[serde(rename = "Fajr")]
    pub fajr: String,
    #[serde(rename = "Dhuhr")]
    pub dhuhr: String,
    #[serde(rename = "Asr")]
    pub asr: String,
    #[serde(rename = "Maghrib")]
    pub maghrib: String,
    #[serde(rename = "Isha")]
    pub isha: String,
}

// 2. Struct untuk membungkus 'timings' yang ada di dalam objek 'data'
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct AladhanData {
    pub timings: PrayerTimings,
}

// 3. Struct utama dari respon JSON paling luar
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct AladhanResponse {
    pub code: u16,
    pub status: String,
    pub data: AladhanData,
}