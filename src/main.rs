// external crates
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// local crates
mod tile;
use tile::Tile;

mod sudoku;
mod board;
mod controls;
mod message;
mod popover;
mod numberInput;

use sudoku::Sudoku;
use crate::board::Board;
use crate::controls::Controls;
use crate::message::{Message, MessageState};
use crate::numberInput::NumberInput;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const GITHUB_LOGO: Asset = asset!("/assets/github-mark.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut board = use_signal(|| Sudoku::new_puzzle(40));
    let mut focused = use_signal(|| None::<(usize, usize)>);
    let mut message = use_signal(|| Message::new());

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        body {
            onclick: move |_| {
                focused.set(None);
            },
            a {
                href: "https://github.com/BenJurewicz/sudoku-solver-gui",
                class: "block m-2.5 text-center underline hover:opacity-50 active:opacity-30 visited:opacity-70",
                img {
                    src: GITHUB_LOGO,
                    alt: "Github logo",
                    class: "inline size-3.5 mx-1"
                }
               "github.com/BenJurewicz/sudoku-solver-gui"
            }
            div {
                class: "flex flex-wrap portrait:flex-col landscape:flex-row justify-center items-center h-screen",
                div {
                class: "flex justify-center items-center flex-col",
                    Board {
                        board: board,
                        focused: focused,
                        message: message,
                    }
                    Controls {
                        board: board,
                        focused: focused,
                        message: message,
                    }
                }
                NumberInput {
                    board: board,
                    focused: focused,
                    message: message,
                }
            }
        }
    }
}
