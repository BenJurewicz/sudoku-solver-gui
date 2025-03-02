// external crates
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// local crates
mod tile;
use tile::Tile;

mod sudoku;
mod board;
mod controls;

use sudoku::Sudoku;
use crate::board::Board;
use crate::controls::Controls;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut board = use_signal(|| Sudoku::new_puzzle(60));
    let mut focused = use_signal(|| None::<(usize, usize)>);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            onclick: move |_| {
                focused.set(None);
            },
            class: "flex justify-center items-center h-screen flex-col",
            Board {
                board: board,
                focused: focused,
            }
            Controls {
                board: board,
                focused: focused,
            }
        }
    }
}
