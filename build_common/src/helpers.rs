use crate::CommandConfig;
use std::env;
use std::path::PathBuf;

fn get_project_root() -> PathBuf {
    PathBuf::from(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap(),
    )
}
fn get_client_path() -> PathBuf {
    get_project_root().join("client")
}

pub fn get_tailwind_commands() -> Vec<CommandConfig> {
    let client = get_client_path();
    let tailwindcss_name = if cfg!(windows) {
        "tailwindcss.cmd"
    } else {
        "tailwindcss"
    };
    let tailwindcss_path = client.join("node_modules/.bin").join(tailwindcss_name);
    let mut commands = if !tailwindcss_path.exists() {
        vec![CommandConfig {
            program: "npm".into(),
            args: vec!["install".into()],
            dir: Some(client.clone()),
            ..Default::default()
        }]
    } else {
        vec![]
    };

    commands.push(CommandConfig {
        program: tailwindcss_path,
        args: vec![
            "-i".into(),
            client.join("styles/input.css").to_string_lossy().into(),
            "-o".into(),
            client.join("styles/output.css").to_string_lossy().into(),
        ],
        dir: Some(client),
        ..Default::default()
    });

    commands
}

pub fn get_stockfish_commands(wasm: bool) -> Vec<CommandConfig> {
    if cfg!(unix) {
        vec![CommandConfig {
            program: get_client_path().join("build-stockfish.sh"),
            args: if wasm { vec![] } else { vec!["--wasm".into()] },
            ..Default::default()
        }]
    } else {
        vec![]
    }
}

pub fn get_trunk_commands() -> Vec<CommandConfig> {
    // Using a different target directory for wasm prevents deadlock when building
    let target_dir = format!(
        "{}/wasm",
        env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into())
    );
    let mut args = vec!["build".into()];
    if env::var("PROFILE") == Ok("release".into()) {
        args.push("--release".into());
    }

    vec![CommandConfig {
        program: PathBuf::from("trunk"),
        args,
        dir: Some(get_project_root()),
        envs: vec![("CARGO_TARGET_DIR".into(), target_dir)],
    }]
}
