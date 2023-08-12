use crate::arrows::{ArrowData, Arrows, ALPHA};
use async_process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use async_std::task::block_on;
use async_std::{io::BufReader, prelude::*};
use chess::color::Color;
use chess::moves::Move;
use dioxus::hooks::UseRef;
use regex::Regex;
use std::io::Result;

const MOVES: usize = 8;
const THREADS: usize = 8;
const DEPTH: usize = 16;

fn get_info<'a>(output: &'a str, key: &'a str) -> Option<&'a str> {
    let re = Regex::new(&format!(r"{key} (\S+)")).unwrap();
    re.captures(output)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
}

async fn send_command(stdin: &mut ChildStdin, command: &str) -> Result<()> {
    stdin.write_all(&format!("{command}\n").into_bytes()).await
}

fn inv_sigmoid(x: f64) -> f64 {
    (x / (1.0 - x)).ln()
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn get_eval(output: &str, player: Color) -> f64 {
    let eval = get_info(output, "score cp")
        .unwrap()
        .parse::<f64>()
        .unwrap()
        / 10.0;
    if player == Color::Black {
        -eval
    } else {
        eval
    }
}

// Makes it so the arrow for the best move has the default ALPHA value
fn eval_to_alpha(eval: f64, evals: &[f64]) -> f64 {
    sigmoid(inv_sigmoid(ALPHA) + eval - evals.iter().max_by(|a, b| a.total_cmp(b)).unwrap())
}

pub async fn run_stockfish() -> Result<Child> {
    let mut cmd = Command::new("client/Stockfish/src/stockfish");
    cmd.stdout(Stdio::piped()).stdin(Stdio::piped());

    log::info!("Starting Stockfish");
    let mut child = cmd.spawn()?;
    let stdin = child.stdin.as_mut().unwrap();

    send_command(stdin, "uci").await?;
    send_command(stdin, &format!("setoption name MultiPV value {MOVES}")).await?;
    send_command(stdin, &format!("setoption name Threads value {THREADS}")).await?;

    Ok(child)
}

pub async fn update_position(fen_str: String, process: UseRef<Option<Child>>) -> Result<()> {
    process.with_mut(|option| -> Result<()> {
        if let Some(process) = option {
            let stdin = process.stdin.as_mut().unwrap();
            block_on(send_command(stdin, "stop"))?;
            block_on(send_command(stdin, &format!("position fen {fen_str}")))?;
            block_on(send_command(stdin, &format!("go depth {DEPTH}")))?;
        }
        Ok(())
    })?;
    Ok(())
}

pub async fn update_analysis_arrows(arrows: &UseRef<Arrows>, stdout: ChildStdout, player: Color) {
    let mut lines = BufReader::new(stdout).lines();
    let mut evals = vec![0.0; MOVES];

    arrows.set(Arrows::new(vec![Move::default(); MOVES]));

    while let Some(Ok(output)) = &lines.next().await {
        if let Some(i) = get_info(output, "multipv") {
            let i = i.parse::<usize>().unwrap() - 1;
            let move_str = get_info(output, " pv").unwrap();
            let eval = get_eval(output, player);
            evals[i] = eval;
            arrows.with_mut(|arrows| {
                arrows.set(
                    i,
                    ArrowData::new(
                        Move::from_lan(move_str).unwrap(),
                        eval_to_alpha(eval, &evals),
                    ),
                )
            });
        }
    }
}
