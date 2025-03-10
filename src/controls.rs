use std::num::NonZeroU8;
use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::message::Message;
use crate::sudoku::Sudoku;
use crate::message::*;
use crate::popover::Popover;

#[component]
pub fn Controls(board: Signal<Sudoku>, focused:Signal<Option<(usize, usize)>>, message: Signal<Message>) -> Element {
    let mut show = use_signal(|| false);

    use_effect(move || {
        if (message.read().state == MessageState::None) {
            show.set(false);
        } else {
            show.set(true);
        }
    });

    rsx! {
        Popover {
            id: "message-popup",
            title: "{message.read().title}",
            show: show,
            class: format!("{}",
                if message.read().state == MessageState::Error {
                    "bg-red-50 text-red-500"
                } else {
                    "bg-green-50 text-green-500"
                }
            ),
            div {
                "{message.read().message}"
            }
        }
        div {
            class: "flex justify-center flex-wrap m-2 md:m-4 lg:m-6",

            button {
                class: "transition text-xl bg-green-500 hover:bg-green-600 active:bg-green-700 text-white font-bold py-2 px-3 m-2 border-solid border-green-800 border-2 rounded-lg",
                onclick: move |_| {
                    board.set(Sudoku::new_puzzle(60));
                },
                "New"
            }

            button {
                class: "transition text-xl bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold py-2 px-4 m-2 border-solid border-blue-800 border-2 rounded-lg",
                onclick: move |_| {
                    if let Err(e) = board.write().solve() {
                        message.write().set(MessageState::Error);
                        } else {
                        message.write().set(MessageState::None);
                    }
                },
                "Solve"
            }

            button {
                class: "transition text-xl bg-red-500 hover:bg-red-600 active:bg-red-700 text-white font-bold py-2 px-4 m-2 border-solid border-red-800 border-2 rounded-lg",
                onclick: move |_| {
                    message.write().set(MessageState::None);
                    board.write().clear();
                },
                "Clear"
            }
        }
    }
}