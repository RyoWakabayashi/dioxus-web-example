use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct GaugeProps<'a> {
    num_blocks: &'a i32,
}

pub fn Gauge<'a>(cx: Scope<'a, GaugeProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "w-[100px] h-[400px]",

            (0..99).map(|index| {
                let class =
                    if &index >= cx.props.num_blocks {
                        "w-[100px] h-[3px] mb-px invisible"
                    } else if index >= 80 {
                        "w-[100px] h-[3px] mb-px bg-[#f00]"
                    } else {
                        "w-[100px] h-[3px] mb-px bg-[#0f0]"
                    };

                rsx! {
                  div {
                      class: "{class}"
                  }
                }
            })
        }
    ))
}
