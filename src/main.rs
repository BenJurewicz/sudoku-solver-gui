// external crates
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// local crates
mod tile;
use tile::Tile;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut board = use_signal(|| [[Some::<u8>(2); 9]; 9]);
    let mut focused = use_signal(|| None::<(usize, usize)>);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            onclick: move |_| {
                focused.set(None);
            },
            class: "flex justify-center items-center h-screen flex-col",
            //TODO: Extract the table into a separate component called board
            table {
            class: "border-collapse border-solid border-3 border-black",
            for y in 0..9 {
                tr{ for x in 0..9 {
                    td {
                        class: format!("p-0 border-solid border {x} {y}",
                                x=if x%3 == 0 && x != 0 {"border-l-3"} else {""},
                                y=if y%3 == 0 && y != 0 {"border-t-3"} else {""}),
                        Tile {
                            board: board,
                            focused: focused,
                            x: x,
                            y: y,
                        }}
                    }}
                }
            }
            div {
                class: "flex justify-center flex-wrap m-2 md:m-4 lg:m-6",
                button {
                    class: "text-xl bg-green-500 hover:bg-green-600 active:bg-green-700 text-white font-bold py-2 px-3 m-2 border-solid border-green-800 border-2 rounded-lg",
                    "Check"
                }
                button {
                    class: "text-xl bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold py-2 px-4 m-2 border-solid border-blue-800 border-2 rounded-lg",
                    "Solve"
                }

                button {
                    popovertarget: "clear-board-conformation",
                    class: "text-xl bg-red-500 hover:bg-red-600 active:bg-red-700 text-white font-bold py-2 px-4 m-2 border-solid border-red-800 border-2 rounded-lg",
                    onclick: move |_| {
                        board.set( [[None; 9]; 9]);
                    },
                    "Clear"
                }
                div {
                    // TODO read more about popovers in dioxus
                    // todo add a popup with "Are you sure you want to clear the board?" and a button to confirm
                    // bug the button should clear ony the editable cells
                    popover: "clear-board-conformation",
                    class: "hidden open:block",
                    id: "clear-board-conformation",
                    "test"
                }
            }
        }
    }
}
