use dioxus::prelude::*;

pub fn Counter(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx!(
        div {

            h1 {
                class: "mb-8 text-4xl font-light",
                "The count is: {count}"
            }

            button {
                class: "mb-4 mr-2 text-white bg-blue-500 border-0 rounded py-1 px-4 focus:outline-none hover:bg-gray-300",
                onclick: move |_| set_count(count - 1),
                "-"
            }

            button {
                class: "mb-4 text-white bg-blue-500 border-0 rounded py-1 px-4 focus:outline-none hover:bg-gray-300",
                onclick: move |_| set_count(count + 1),
                "+"
            }

            crate::components::gauge::Gauge {
                num_blocks: count
            }
        }
    ))
}
