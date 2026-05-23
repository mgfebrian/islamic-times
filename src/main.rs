use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::pages::home::Home;

mod api;
mod components;
mod models; 
mod pages;
mod utils;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/prayer-times")]
    PrayerTimes,
    #[at("/fasting-tracker")]
    FastingTracker,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogDetail {id: String},
    #[at("/todo-list")]
    TodoList,
    #[at("/calendar")]
    Calendar,
    #[at("/about")]
    About,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::PrayerTimes => html! { <h1>{ "PrayerTimes" }</h1> },
        Route::FastingTracker => html! { <h1>{ "FastingTracker" }</h1> },
        Route::Blog => html! { <h1>{ "Blog" }</h1> },
        Route::BlogDetail {id} => html! { <h1>{ format!("BlogDetail {}", id) }</h1> },
        Route::TodoList => html! { <h1>{ "TodoList" }</h1> },
        Route::Calendar => html! { <h1>{ "Calendar" }</h1> },
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::Settings => html! { <h1>{ "Settings" }</h1> },
        Route::NotFound => html! { <h1>{ "NotFound" }</h1> },
    }
}

#[component]
fn App() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Navbar />
                <Switch<Route> render={switch} />
            </BrowserRouter>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}