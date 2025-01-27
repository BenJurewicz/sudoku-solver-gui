#![allow(non_snake_case)]

// external crates
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
// local crates
mod tile;
use tile::Tile;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="assets/tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
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
    // let mut board = vec![vec![use_signal(|| None::<u8>); 9]; 9];
    let mut test = use_signal(|| None::<u8>);

    rsx! {
        div { class: "grid grid-rows-9 grid-cols-9",
            for _ in 0..(9*9) {
                // Tile { value: board[i / 9][i % 9] }
                Tile {
                    value: test(),
                    oninput: move |e : FormEvent| {
                        test.set(Some (e.value().pop().unwrap_or('0').to_digit(10).unwrap_or(0) as u8));
                    }
                }
            }
        }
    }
}
