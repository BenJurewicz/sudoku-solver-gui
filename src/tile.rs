use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Tile(board: Signal<[[Option<u8>; 9]; 9]>, focused:Signal<Option<(usize, usize)>>, x: usize, y: usize) -> Element {
    let handleInput = move |e : KeyboardEvent| {
        if let Key::Character(c) = e.key() {
            let num =
                c.parse::<u8>().ok()
                .and_then(|n| if n == 0 { None } else { Some(n) });
            board.write()[x][y] = num;
            // needs_update(); // was needed before, keeping commented out coz it's hard to find in docs
        }
    };

    // TODO this sohuld be use memo or wathever its called
    let shown_value = if let Some(v) = board()[x][y] {
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

    rsx! {
        div {
            tabindex: "0",
            class: format!("font-sans font-medium flex items-center justify-center w-12 h-12 hover:bg-gray-300 focus:bg-gray-400 {}",
                if focus_neighbour {"bg-gray-200"} else {""}),
            onkeydown: handleInput,
            onclick: move |e| {
                e.stop_propagation();
                focused.set(Some((x, y)));
            },
            "{shown_value}"
        }
    }
}