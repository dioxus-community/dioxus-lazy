use crate::{use_list::UseList, Factory};
use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps<'a, F, G> {
    /// Length of the list.
    pub len: usize,

    /// Size of the container.
    pub size: f64,

    /// Size of each item.
    pub item_size: f64,

    /// Function to create a new item.
    pub make_item: F,

    /// Function to create a new value.
    pub make_value: G,

    /// Event handler for scroll events.
    pub onscroll: Option<EventHandler<'a>>,
}

/// Virtualized list component.
#[allow(non_snake_case)]
pub fn List<'a, T: 'static, F, G>(cx: Scope<'a, ListProps<'a, F, G>>) -> Element<'a>
where
    F: Fn(&T) -> Element<'a>,
    G: Factory<Item = T> + Clone + 'static,
{
    let list = UseList::builder()
        .len(cx.props.len)
        .size(cx.props.size)
        .item_size(cx.props.item_size)
        .use_list(cx, cx.props.make_value.clone());

    let values_ref = list.values.read();
    let rows = values_ref.iter().enumerate().map(|(idx, value)| {
        let top = (list.start() + idx) as f64 * *list.item_size.read();
        render!(
            div {
                key: "{top}",
                position: "absolute",
                top: "{top}px",
                left: 0,
                width: "100%",
                height: "{list.item_size}px",
                overflow: "hidden",
                (cx.props.make_item)(value)
            }
        )
    });

    let size = *list.size.read();
    render!(
        div {
            height: "{size}px",
            overflow: "scroll",
            onmounted: move |event| list.mounted.onmounted(event),
            onscroll: move |_| {
                list.scroll();
                if let Some(handler) = &cx.props.onscroll {
                    handler.call(())
                }
            },
            div {
                position: "relative",
                height: "{list.item_size * cx.props.len as f64}px",
                overflow: "hidden",
                rows
            }
        }
    )
}
