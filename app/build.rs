mod build_utils;

use std::env;

use build_utils::*;

fn main() {
    if env::var("TARGET").map_or(false, |target| target.contains("wasm32")) {
        return;
    }

    println!("cargo:rerun-if-changed=../images/boards");

    let mut commands = get_tailwind_commands();
    if cfg!(feature = "ssr") {
        commands.extend(get_trunk_commands());
    } else {
        commands.extend(get_stockfish_commands());
    }
    CommandConfig::run_build_commands(&commands);

    write_all_complimentary_colors().expect("failed to write complimentary colors");
}
