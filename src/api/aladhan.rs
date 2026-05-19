use gloo_net::http::Request;
use crate::models::prayer::AladhanResponse;

pub async fn fetch_prayer_times(
    lat: f64, 
    lon: f64, 
    date: &str
) -> Result<AladhanResponse, gloo_net::Error> {
    
    // URL dinamis. Method=20 adalah standar Kementerian Agama RI.
    let url = format!(
        "https://api.aladhan.com/v1/timings/{}?latitude={}&longitude={}&method=20",
        date, lat, lon
    );

    // Melakukan HTTP GET Request ke Aladhan API
    let response = Request::get(&url).send().await?;

    // Deserialisasi otomatis JSON ke dalam Struct Rust kita
    let data: AladhanResponse = response.json().await?;
    
    Ok(data)
}