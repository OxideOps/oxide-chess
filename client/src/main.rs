#[feature(async_fn_in_trait)]
use common::args::*;

pub fn main() {
    dioxus_logger::init(Args::parse().log_level).expect("Failed to initialize dioxus logger");
    client::launch();
}
