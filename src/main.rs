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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut board = use_signal(|| Sudoku::new_puzzle(60));
    let mut focused = use_signal(|| None::<(usize, usize)>);
    let mut message = use_signal(|| Message::new());

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            onclick: move |_| {
                focused.set(None);
            },
            class: "flex flex-wrap flex-col justify-center items-center h-screen",
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
