use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Tile(value: Signal<Option<u8>>) -> Element {
    let shown_value = if let Some(v) = value() {
        v.to_string()
    } else {
        String::from("")
    };
    rsx! {
        div {
            tabindex: "0",
            class: "bg-pink-100 w-10 h-10 border-solid border text-center",
            onkeydown: move |e| {
                if let Key::Character(c) = e.key() {
                    value.set(
                        c.parse::<u8>().ok()
                    );
                    needs_update();
                }
            },
            "{shown_value}"
        }
        // input { r#type: "number", min:"1", max:"9", size:"1", value: value, class: "border outline-none bg-pink-100 w-10 h-10 text-center caret-transparent",
        //     // oninput: move |e| { oninput.call(e); needs_update(); }
        //     oninput: move |e : FormEvent| {
        //         value.set(
        //             e.value().pop().
        //             and_then(|c| {info!("c: {}", c);
        //                 c.to_digit(10)}).
        //             and_then(|c| Some(c as u8))
        //         );
        //         needs_update();
        //     }
        // }
    }
}