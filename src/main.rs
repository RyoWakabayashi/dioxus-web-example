use dioxus::prelude::*;

pub mod components {
    pub mod counter;
    pub mod gauge;
}

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            class: "flex justify-center p-2 mt-5",
            components::counter::Counter {}
        }
    ))
}
