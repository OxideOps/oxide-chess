use chess::app::App;

pub fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch_cfg(App, dioxus_web::Config::new().hydrate(true));
    #[cfg(feature = "server")]
    {
        use axum::{extract::WebSocketUpgrade, routing::get};
        use chess::server::game_socket::{handler, PlayerConnections, GAME_ID};
        use dioxus_fullstack::prelude::*;
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let connections: PlayerConnections = Default::default();
                let addr = "[::]:8080".parse().unwrap();
                println!("listening on {}", addr);
                axum::Server::bind(&addr)
                    .serve(
                        axum::Router::new()
                            .route(
                                &*format!("/{GAME_ID}"),
                                get(|ws: WebSocketUpgrade| handler(ws, connections)),
                            )
                            .serve_dioxus_application("", ServeConfigBuilder::new(App, ()))
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            });
    }
    #[cfg(feature = "desktop")]
    {
        use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
        const WINDOW_SIZE: u32 = 800;
        dioxus_desktop::launch_cfg(
            App,
            Config::new().with_window(WindowBuilder::new().with_title("Chess").with_inner_size(
                LogicalSize {
                    width: WINDOW_SIZE,
                    height: WINDOW_SIZE,
                },
            )),
        );
    }
}
