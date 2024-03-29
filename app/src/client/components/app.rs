use chess::{Color, Game};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::super::{
    router::Route,
    shared_states::{Analyze, BoardSize, GameId, Perspective, Settings},
    stockfish::Eval,
};

const WIDGET_HEIGHT: u32 = 800;

pub(crate) fn App(cx: Scope) -> Element {
    log::info!("app launched");

    use_shared_state_provider(cx, || Eval::Centipawns(0));
    use_shared_state_provider(cx, || GameId(None));
    use_shared_state_provider(cx, Game::new);
    use_shared_state_provider(cx, || BoardSize(WIDGET_HEIGHT));
    use_shared_state_provider(cx, || Perspective(Color::White));
    use_shared_state_provider(cx, || Analyze(false));
    use_shared_state_provider(cx, Settings::new);

    cx.render(rsx! {
        style { {include_str!("../../../styles/output.css")} }
        Router::<Route> {}
    })
}
