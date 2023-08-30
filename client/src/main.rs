use client::app::App;
use common::args::*;

pub fn main() {
    dioxus_logger::init(Args::parse().log_level).expect("Failed to initialize dioxus logger");
    #[cfg(target_arch = "wasm32")]
    {
        log::info!("web launching");
        dioxus_web::launch(App);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use dioxus_desktop::{Config, WindowBuilder};
        use dioxus_fullstack::prelude::server_fn::set_server_url;
        set_server_url("https://muddy-fog-684.fly.dev");
        log::info!("desktop launching");
        dioxus_desktop::launch_cfg(
            App,
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title("Chess")
                        .with_maximized(true),
                )
                .with_disable_context_menu(true),
        );
    }
}
