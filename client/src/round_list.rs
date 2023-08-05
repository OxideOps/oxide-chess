use chess::game::Game;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct RoundListProps<'a> {
    game: &'a UseRef<Game>,
}

pub fn RoundList<'a>(cx: Scope<'a, RoundListProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "rounds-container",
            p { "Rounds:" },
            cx.props.game.with(|game| {
                game.get_rounds_info().into_iter().enumerate().map(|(i, info)| {
                    let fill = info.2;
                    let classes = if fill {
                        "mb-4 bg-gray-300/50"
                    } else {
                        "mb-4"
                    };

                    rsx! {
                        div {
                            class: "{classes}",
                            tr {
                                td {
                                    class: "pr-4",
                                    "{i + 1}."
                                }
                                td {
                                    class: "pr-4",
                                    "{info.0}"
                                }
                                td {
                                    class: "pr-4",
                                    "{info.1}"
                                }
                            }
                        }
                    }
                })
            })
        }
    })
}
