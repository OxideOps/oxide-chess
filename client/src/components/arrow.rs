use std::f64::consts::PI;

use dioxus::{html::geometry::ClientPoint, prelude::*};

use super::get_center;
use crate::{arrows::ArrowData, components::board::BoardProps};

// the following are measured relative to the board size
const HEAD: f64 = 1.0 / 30.0; // size of arrow head
const WIDTH: f64 = 1.0 / 80.0; // width of arrow body
const OFFSET: f64 = 1.0 / 20.0; // how far away from the middle of the starting square

fn get_color(alpha: f64) -> String { format!("rgba(27, 135, 185, {})", alpha) }

fn get_angle_from_vertical(from: &ClientPoint, to: &ClientPoint) -> f64 {
    (to.y - from.y).atan2(to.x - from.x) + PI / 2.0
}

#[component]
pub(crate) fn Arrow<'a>(cx: Scope, data: ArrowData, board_props: &'a BoardProps) -> Element {
    if !data.has_length() {
        return None;
    }

    let from = get_center(board_props, &data.mv.from);
    let to = get_center(board_props, &data.mv.to);

    let h = HEAD * board_props.size as f64;
    let w = WIDTH * board_props.size as f64;
    let o = OFFSET * board_props.size as f64;

    let angle = get_angle_from_vertical(&from, &to);
    let sin = angle.sin();
    let cos = angle.cos();

    let x0 = to.x as u32;
    let y0 = to.y as u32;

    let x1 = (to.x + h * cos - h * sin) as u32;
    let y1 = (to.y + h * sin + h * cos) as u32;

    let x2 = (to.x + w * cos - h * sin) as u32;
    let y2 = (to.y + w * sin + h * cos) as u32;

    let x3 = (from.x + w * cos + o * sin) as u32;
    let y3 = (from.y + w * sin - o * cos) as u32;

    let x4 = (from.x - w * cos + o * sin) as u32;
    let y4 = (from.y - w * sin - o * cos) as u32;

    let x5 = (to.x - w * cos - h * sin) as u32;
    let y5 = (to.y - w * sin + h * cos) as u32;

    let x6 = (to.x - h * cos - h * sin) as u32;
    let y6 = (to.y - h * sin + h * cos) as u32;

    cx.render(rsx! {
        svg {
            class: "absolute pointer-events-none",
            style: "z-index: 3",
            height: "{board_props.size}",
            width: "{board_props.size}",
            polygon {
                class: "absolute pointer-events-none",
                points: "{x0},{y0}, {x1},{y1} {x2},{y2} {x3},{y3} {x4},{y4} {x5},{y5} {x6},{y6}",
                fill: "{get_color(data.alpha)}"
            }
        }
    })
}
