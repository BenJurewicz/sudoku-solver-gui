use crate::message::Message;
use crate::message::*;
use crate::sudoku::Sudoku;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use std::num::NonZeroU8;

#[component]
pub fn NumberInput(board: Signal<Sudoku>, focused: Signal<Option<(usize, usize)>>, message: Signal<Message>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-3 gap-[5%] mx-10 size-40 md:size-50 max-w justify-center items-center",
            for i in 1..=9 {
                button {
                    disabled: focused.read().map_or(false, |(fx, fy)| board.read().is_read_only(fx, fy)),
                    class: "transition text-3xl md:text-4xl aspect-square bg-gray-500 text-white font-bold border-solid border-gray-800 border-2 rounded-lg \
                            enabled:hover:bg-gray-600 enabled:active:bg-gray-700 \
                            disabled:opacity-40",
                    onclick: move |e| {
                        e.stop_propagation();
                        let selected = match focused.read().clone() {
                            Some((fx, fy)) if !board.read().is_read_only(fx, fy) => Some((fx, fy)),
                            _ => None
                        };
                        if let Some((fx, fy)) = selected {
                            board.write().set_cell(fx, fy, Some(NonZeroU8::new(i).unwrap()));
                            if(board.read().check()){
                                message.write().set(MessageState::Solved);
                            }
                        }
                    },
                    "{i}"
                }
            }
        }
    }
}
