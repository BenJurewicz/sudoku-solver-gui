#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Tile(value: Option<u8>, oninput: EventHandler<FormEvent>) -> Element {
    let showedValue = if let Some(v) = value {
        v.to_string()
    } else {
        ' '.to_string()
    };
    // let inputHandler = move |evt: Event<FormData>| {
    //     value = evt.value().pop().unwrap_or('0').to_digit(10).unwrap_or(0) as u8;
    // };

    rsx! {
        input { r#type: "number", min:"1", max:"9", size:"1", value: "{showedValue}", class: "bg-pink-600",
            oninput: move |e| { oninput.call(e); needs_update(); }
        }
    }
}
