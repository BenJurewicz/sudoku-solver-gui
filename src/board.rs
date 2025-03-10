use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::message::Message;
use crate::sudoku::Sudoku;
use crate::tile::Tile;

#[component]
pub fn Board(board: Signal<Sudoku>, focused:Signal<Option<(usize, usize)>>, message: Signal<Message>) -> Element {
    rsx! {
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
                        message: message,
                        x: x,
                        y: y,
                    }}
                }}
            }
        }
    }
}