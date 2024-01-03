use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::client::router::Route;

#[component]
pub(crate) fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            div {
                class: "nav-bar",
                ul {
                    class: "flex",
                    li {
                        Link {
                            class: "nav-link",
                            to: Route::Widget {},
                            "Game"
                        }
                    }
                    li {
                        Link {
                            class: "nav-link",
                            to: Route::Settings {},
                            "Settings"
                        }
                    }
                    li {
                        Link {
                            class: "nav-link",
                            to: Route::SignUp {},
                            "Sign Up"
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
pub(crate) fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "The page you requested doesn't exist: {route:?}." }
    }
}
