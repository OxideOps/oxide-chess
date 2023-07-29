use chess::game::Game;
use chess::moves::Move;
use dioxus::prelude::*;
use futures::executor;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::sync::RwLock;
use tokio_tungstenite_wasm::{connect, Message, WebSocketStream};
use url::Url;

use crate::widget::WidgetProps;

type WriteStream = SplitSink<WebSocketStream, Message>;
type ReadStream = SplitStream<WebSocketStream>;

const GAME_ID: u32 = 1234;
static SOCKET_CREATED: RwLock<bool> = RwLock::new(false);

fn init_streams() -> (Option<WriteStream>, Option<ReadStream>) {
    if !*SOCKET_CREATED.read().unwrap() {
        let (write, read) = executor::block_on(connect(
            Url::parse(&format!("ws://muddy-fog-684.fly.dev/game/{GAME_ID}")).unwrap(),
        ))
        .unwrap()
        .split();
        *SOCKET_CREATED.write().unwrap() = true;
        (Some(write), Some(read))
    } else {
        (None, None)
    }
}

async fn write_to_socket(mut rx: UnboundedReceiver<Move>, write_stream: Option<WriteStream>) {
    if let Some(mut socket) = write_stream {
        while let Some(mv) = rx.next().await {
            log::info!("Sending move {mv:?}");
            socket
                .send(Message::Text(serde_json::to_string(&mv).unwrap()))
                .await
                .unwrap();
        }
    }
}

async fn read_from_socket(read_stream: Option<ReadStream>, game: UseRef<Game>) {
    if let Some(mut stream) = read_stream {
        while let Some(message) = stream.next().await {
            let data = message.unwrap().into_text().unwrap();
            let mv: Move =
                serde_json::from_str(&data).expect("Failed to read move from remote player.");
            log::info!("Got move {mv:?}");
            game.with_mut(|game| game.move_piece(mv.from, mv.to).ok());
        }
    }
}

pub fn create_game_socket<'a>(
    cx: Scope<'a, WidgetProps>,
    game: &UseRef<Game>,
) -> &'a Coroutine<Move> {
    let (write_stream, read_stream) = init_streams();
    use_coroutine(cx, |_rx: UnboundedReceiver<()>| {
        read_from_socket(read_stream, game.to_owned())
    });
    use_coroutine(cx, |rx: UnboundedReceiver<Move>| {
        write_to_socket(rx, write_stream)
    })
}