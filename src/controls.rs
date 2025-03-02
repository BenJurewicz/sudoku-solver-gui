use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::sudoku::Sudoku;

#[component]
pub fn Controls(board: Signal<Sudoku>, focused:Signal<Option<(usize, usize)>>) -> Element {
    rsx! {
        div {
            class: "flex justify-center flex-wrap m-2 md:m-4 lg:m-6",

            button {
                class: "text-xl bg-green-500 hover:bg-green-600 active:bg-green-700 text-white font-bold py-2 px-3 m-2 border-solid border-green-800 border-2 rounded-lg",
                onclick: move |_| {
                    board.set(Sudoku::new_puzzle(60));
                },
                "New"
            }

            button {
                class: "text-xl bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold py-2 px-4 m-2 border-solid border-blue-800 border-2 rounded-lg",
                "Solve"
            }

            button {
                popovertarget: "clear-board-conformation",
                class: "text-xl bg-red-500 hover:bg-red-600 active:bg-red-700 text-white font-bold py-2 px-4 m-2 border-solid border-red-800 border-2 rounded-lg",
                onclick: move |_| {
                    board.write().clear();
                    focused.set(None);
                },
                "Clear"
            }

            div {
                // TODO read more about popovers in dioxus
                // todo add a popup with "Are you sure you want to clear the board?" and a button to confirm
                popover: "clear-board-conformation",
                class: "hidden open:block",
                id: "clear-board-conformation",
                "test"
            }
        }
    }
}