use std::num::NonZeroU8;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::message::{Message, MessageState};
use crate::sudoku::Sudoku;

#[component]
pub fn Tile(board: Signal<Sudoku>, focused:Signal<Option<(usize, usize)>>, message: Signal<Message>, x: usize, y: usize) -> Element {
    let handleInput = move |e : KeyboardEvent| {
        if board.read().is_read_only(x, y) {
            return;
        }
        if let Key::Character(c) = e.key() {
            let num =
                c.parse::<u8>().ok()
                .and_then(|n|
                    if n == 0 {
                        None
                    } else {
                        Some(NonZeroU8::new(n).unwrap())
                    });

            board.write().set_cell(x, y, num);
            if(board.read().check()){
                message.write().set(MessageState::Solved);
            }
            // needs_update(); // was needed before, keeping commented out coz it's hard to find in docs
        } else {
            // Any key clears
            // It's here mainly for the backspace key to delete the value
            board.write().set_cell(x, y, None);
        }
    };

    let shown_value = use_memo(move || {
        if let Some(v) = board.read().get_cell(x, y) {
            v.to_string()
        } else {
            String::from("")
        }});

    let is_focused = focused.read().map_or(false, |(fx, fy)| x == fx && y == fy);

    let is_focused_neighbour = match focused() {
        Some((fx, fy)) => {
            // Checks whether the focused cell has direct influence on
            // the value of the current cell
            x == fx || y == fy || (fx/3==x/3 && fy/3==y/3)
        },
        _ => false
    };

    let is_read_only = board.read().is_read_only(x, y);

    let same_as_focused = match focused() {
        Some((fx, fy)) => board.read().get_cell(x, y) == board.read().get_cell(fx, fy),
        _ => false
    };

    rsx! {
        div {
            tabindex: "0",
            // We want the board to take up around 2/3 of the screen and sudoku grid is 9x9 so:
            // x * 9 / 100 = 2/3
            // x = 7,4 ≈ 2/3 * 100/9
            class: format!("size-[9vmin] sm:size-[7.4vmin] text-[6vmin] md:text-[4.5vmin] lg:text-[3vmin] text-center select-none cursor-pointer {} {}",
                if is_read_only && !is_focused {"bg-zinc-600 text-white font-bold"}
                else if is_read_only && is_focused {"bg-zinc-700 text-white font-bold"}
                else if is_focused {"bg-gray-400 font-semibold"}
                else if is_focused_neighbour {"bg-gray-200"}
                else {"hover:bg-gray-300 active:bg-gray-500"},

                if is_focused {""}
                else if same_as_focused && is_focused_neighbour {"!text-red-400"}
                else if same_as_focused && !is_focused_neighbour {"!text-green-400"}
                else {""}),

            onkeydown: handleInput,
            onclick: move |e| {
                // Prevents the item from being unfocused by onclick handler in root div
                e.stop_propagation();
                // if(!is_read_only) {
                    focused.set(Some((x, y)));
                // }
            },
            "{shown_value}"
        }
    }
}