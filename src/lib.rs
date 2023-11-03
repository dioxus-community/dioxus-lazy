use dioxus::prelude::*;

#[component]
pub fn List<'a, F: Fn(usize) -> Element<'a>>(
    cx: Scope<'a>,
    len: usize,
    item_height: f64,
    item: F,
) -> Element<'a> {
    let rows = (0..*len).map(|idx| {
        render!(div {
            width: "100%",
            height: "{item_height}px",
            overflow: "hidden",
            item( idx)
        })
    });
    render!(div {
        rows
    })
}
