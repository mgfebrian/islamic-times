use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct AladhanResponse {
    pub code: u16,
    pub status: String,
    pub data: AladhanData,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct AladhanData {
    pub timings: PrayerTimings,
    pub date: CurrentDate,
    pub meta: MetaData
}

// ==========================
// START OF TIMINGS REPONSE
// ==========================
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct PrayerTimings {
    #[serde(rename = "Lastthird")]
    pub lastthird: String,
    #[serde(rename = "Fajr")]
    pub fajr: String,
    #[serde(rename = "Sunrise")]
    pub sunrise: String,
    #[serde(rename = "Dhuhr")]
    pub dhuhr: String,
    #[serde(rename = "Asr")]
    pub asr: String,
    #[serde(rename = "Maghrib")]
    pub maghrib: String,
    #[serde(rename = "Isha")]
    pub isha: String,
}
// ==========================
// END OF TIMINGS REPONSE
// ==========================

// ==========================
// START OF DATES REPONSE
// ==========================
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CurrentDate {
    pub readable: String,
    pub hijri: HijriDate,
    pub gregorian: GregorianDate
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct HijriDate {
    pub day: String,
    pub weekday: HijriWeekday,
    pub month: HijriMonth,
    pub year: String,
    pub designation: HijriDesignation
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct HijriWeekday {
    pub en: String,
    pub ar: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct HijriMonth {
    pub en: String,
    pub ar: String
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct HijriDesignation {
    pub abbreviated: String,
    pub expanded: String
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct GregorianDate {
    pub day: String,
    pub weekday: GregorianWeekday,
    pub month: GregorianMonth,
    pub year: String,
    pub designation: GregorianDesignation
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct GregorianWeekday {
    pub en: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct GregorianMonth {
    pub en: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct GregorianDesignation {
    pub abbreviated: String,
    pub expanded: String,
}
// ==========================
// END OF DATES REPONSE
// ==========================

// ==========================
// START OF META REPONSE
// ==========================
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct MetaData {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub method: MetaMethod,
    #[serde(rename = "latitudeAdjustmentMethod")]
    pub latitude_adjustment_method: String,
    #[serde(rename = "midnightMode")]
    pub midnight_mode: String,
    pub school: String,

}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct MetaMethod {
    pub name: String,
}



// ==========================
// END OF DATES REPONSE
// ==========================