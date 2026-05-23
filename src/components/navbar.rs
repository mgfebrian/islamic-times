use yew::prelude::*;
// 1. Wajib import komponen dari yew-router
use yew_router::prelude::*; 

use crate::Route;

#[derive(Clone)]
pub struct RouteMenu {
    pub route: Route,
    pub name: AttrValue,
}

#[component(Navbar)]
pub fn navbar() -> Html {
    let menus: [RouteMenu; 7] = [
        RouteMenu {
            route: Route::Home,
            name: AttrValue::from("Home"),
        },
        RouteMenu {
            route: Route::PrayerTimes,
            name: AttrValue::from("Prayer"),
        },
        RouteMenu {
            route: Route::TodoList,
            name: AttrValue::from("To Do"),
        },
        RouteMenu {
            route: Route::Calendar,
            name: AttrValue::from("Calendar"),
        },
        RouteMenu {
            route: Route::FastingTracker,
            name: AttrValue::from("Fasting"),
        },
        RouteMenu {
            route: Route::Blog,
            name: AttrValue::from("Blog"),
        },
        RouteMenu {
            route: Route::Settings,
            name: AttrValue::from("Settings"),
        },
    ];

    html! {
        <div class="navbar bg-base-100 shadow-sm">
            <div class="flex-1">
                <a class="btn btn-ghost text-xl">
                    <Link<Route> to={menus[0].route.clone()}>
                        { "Shalat Times" }
                    </Link<Route>>
                </a>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1">
                    for menu in menus {    
                        <li>
                            <Link<Route> to={menu.route.clone()}>
                                { &menu.name }
                            </Link<Route>>
                        </li>
                    }
                </ul>
            </div>
        </div>
    }
}