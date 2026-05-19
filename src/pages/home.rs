use yew::prelude::*;

use crate::{
    api::aladhan::fetch_prayer_times, models::prayer::PrayerTimings, utils::date::get_today_string,
};

#[function_component(Home)]
pub fn home() -> Html {
    let prayer_data = use_state(|| None::<PrayerTimings>);
    let error_msg = use_state(|| None::<String>);

    {
        let prayer_data = prayer_data.clone();
        let error_msg = error_msg.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let lat = -6.556414171422679;
                let lon = 107.7913168398452;
                let date = get_today_string();

                match fetch_prayer_times(lat, lon, &date).await {
                    Ok(response) => {
                        prayer_data.set(Some(response.data.timings));
                    }
                    Err(err) => {
                        error_msg.set(Some(err.to_string()));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div class="min-h-screen bg-base-200">

            <div class="container mx-auto px-4 flex justify-center pb-10">
                <div class="card w-full max-w-2xl bg-base-100 shadow-xl">
                    <div class="card-body">

                        // Menampilkan Error jika ada
                        if let Some(err) = &*error_msg {
                            <div class="alert alert-error shadow-lg mb-4">
                                <div>
                                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                    <span>{ format!("Gagal mengambil data: {}", err) }</span>
                                </div>
                            </div>
                        }

                        // Menampilkan data jika sudah selesai loading (isinya Some)
                        if let Some(timings) = &*prayer_data {
                            <h2 class="text-2xl font-bold text-center mb-6 text-primary">
                                { "Waktu Sholat Hari Ini" }
                            </h2>

                            // Grid layout untuk jadwal sholat (DaisyUI / Tailwind)
                            <div class="grid grid-cols-2 md:grid-cols-5 gap-4">

                                // Subuh
                                <div class="stat bg-neutral text-neutral-content rounded-box place-items-center">
                                    <div class="stat-title text-neutral-content opacity-70">{ "Subuh" }</div>
                                    <div class="stat-value text-xl">{ &timings.fajr }</div>
                                </div>

                                // Dzuhur
                                <div class="stat bg-neutral text-neutral-content rounded-box place-items-center">
                                    <div class="stat-title text-neutral-content opacity-70">{ "Dzuhur" }</div>
                                    <div class="stat-value text-xl">{ &timings.dhuhr }</div>
                                </div>

                                // Ashar
                                <div class="stat bg-neutral text-neutral-content rounded-box place-items-center">
                                    <div class="stat-title text-neutral-content opacity-70">{ "Ashar" }</div>
                                    <div class="stat-value text-xl">{ &timings.asr }</div>
                                </div>

                                // Maghrib (Beri warna berbeda agar menonjol)
                                <div class="stat bg-primary text-primary-content rounded-box place-items-center shadow-lg transform scale-105">
                                    <div class="stat-title text-primary-content opacity-80 font-bold">{ "Maghrib" }</div>
                                    <div class="stat-value text-2xl">{ &timings.maghrib }</div>
                                </div>

                                // Isya
                                <div class="stat bg-neutral text-neutral-content rounded-box place-items-center">
                                    <div class="stat-title text-neutral-content opacity-70">{ "Isya" }</div>
                                    <div class="stat-value text-xl">{ &timings.isha }</div>
                                </div>

                            </div>
                        } else if error_msg.is_none() {
                            // Loading State (jika belum error dan belum ada data)
                            <div class="flex flex-col items-center justify-center py-10">
                                <span class="loading loading-bars loading-lg text-primary"></span>
                                <p class="mt-4 text-gray-500 font-semibold">{ "Mengambil jadwal sholat..." }</p>
                            </div>
                        }

                    </div>
                </div>
            </div>
        </div>
    }
}
