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

/*
Source:
    https://dioxuslabs.com/learn/0.5/cookbook/state/custom_hooks#custom-hook-logic
General idea for handling the sudoku state
    Create a custom hook that will return a signal wrapper with these methods
    - get_cell( point: Point<u8, u8> ) -> u8 (it has to be more than u8, some kind of use_memo since it's just a view)
        returns the value of the given cell coordinate; useful for rendering the cell view (currently named tile)
    - get_whole() -> [[u8; 9] 9]
        returns the whole sudoku array; will be useful when implementing the button to solve the sudoku
    - get_update_func( point: Point<u8, u8> ) -> Fn<takes in u8 (don't remember the syntax)>
        takes in the coordinate of the cell
        returns a method for updating the cell with the given coordinate
        the method takes in the value that the cell should be changed to and returns nothing
        useful for implementing the oninput for the cell view (currently named tile)
    Note: maybe use x: u8, y:u8 instead of point?

    how to use in code:
        fn App() -> Element {
            let mut sudoku = use_sudoku();

            rsx! {
                Tile {
                    value: sudoku.get_cell(Point::new(0,0)),
                    oninput: sudoku.get_update_func(Point::new(0,0))
                }
        }
*/
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
