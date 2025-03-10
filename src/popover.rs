use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Popover(id: String, title: String, show: Signal<bool>, class: Option<String>, children: Element) -> Element {
    if(*show.read()){
        eval(format!("document.getElementById(\"{}\").showPopover()", id).as_str());
    } else {
        eval(format!("document.getElementById(\"{}\").hidePopover()", id).as_str());
    }
    rsx! {
        div {
            popover: "auto",
            class: format!("m-auto max-w-[25rem] w-3/4 fixed px-5 pt-1 pb-1.5 text-current/70 bg-gray-100 rounded-lg {}", class.unwrap_or_default()),
            id: {id},
            div {
                class: "-mx-1.5 flex items-center justify-between font-semibold text-lg",
                {title},
                // X close button
                svg {
                    onclick: move |_| {
                        show.set(false);
                    },
                    class: "hover:opacity-50 active:opacity-30 size-6",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M6 18 18 6M6 6l12 12"
                    }
                }
            }
            hr {
                class: "-mx-1 my-1 border-solid border-current border-1 rounded-full"
            }
            {children}
        }
    }
}