use js_sys::Date;

pub fn get_today_string() -> String {
    // Meminta waktu saat ini langsung dari browser pengguna
    let date = Date::new_0();
    
    let day = date.get_date();
    let month = date.get_month() + 1; // Di JavaScript, bulan dimulai dari 0 (Januari = 0)
    let year = date.get_full_year();

    // Format menjadi DD-MM-YYYY (tambahkan angka 0 di depan jika di bawah 10)
    format!("{:02}-{:02}-{}", day, month, year)
}