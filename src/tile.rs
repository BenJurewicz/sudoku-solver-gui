use std::num::NonZeroU8;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::sudoku::Sudoku;

#[component]
pub fn Tile(board: Signal<Sudoku>, focused:Signal<Option<(usize, usize)>>, x: usize, y: usize) -> Element {
    let handleInput = move |e : KeyboardEvent| {
        if let Key::Character(c) = e.key() {
            let num =
                c.parse::<u8>().ok()
                .and_then(|n| if n == 0 { None } else { Some(NonZeroU8::new(n).unwrap()) });
            board.write().set_cell(x, y, num);
            // needs_update(); // was needed before, keeping commented out coz it's hard to find in docs
        }
    };

    // TODO this sohuld be use memo or whatever its called
    let shown_value = if let Some(v) = board.read().get_cell(x, y) {
        v.to_string()
    } else {
        String::from("")
    };

    let focus_neighbour = match focused() {
        Some((fx, fy)) => {
            // Checks whether the focused cell has direct influence on
            // the value of the current cell
            x == fx || y == fy || (fx/3==x/3 && fy/3==y/3)
        },
        _ => false
    };
    let is_read_only = board.read().is_read_only(x, y);

    rsx! {
        div {
            tabindex: "0",
            // We want the board to take up around 2/3 of the screen and sudoku grid is 9x9 so:
            // x * 9 / 100 = 2/3
            // x = 7,4 ≈ 2/3 * 100/9
            class: format!("size-[9vmin] sm:size-[7.4vmin] flex items-center justify-center font-stretch-100% hover:bg-gray-300 active:bg-gray-500 focus:bg-gray-400 focus:font-semibold text-[3vmin] {}",
                if is_read_only {"!bg-zinc-600 !text-white !font-bold"} else if focus_neighbour {"bg-gray-200"} else {""}),
            onkeydown: handleInput,
            onclick: move |e| {
                // Prevents the item from being unfocused by onclick handler in root div
                e.stop_propagation();
                if(!is_read_only) {
                    focused.set(Some((x, y)));
                }
            },
            "{shown_value}"
        }
    }
}