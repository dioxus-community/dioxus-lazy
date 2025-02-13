use dioxus_lib::prelude::*;

use crate::{
    lazy::{Lazy, Values},
    use_list::UseList,
};

#[derive(Props, Clone)]
pub struct ListProps<F: 'static, G: 'static>
where
    F: Clone,
    G: Clone,
{
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
    pub onscroll: Option<EventHandler>,
}

impl<F: 'static, G: 'static> PartialEq for ListProps<F, G>
where
    F: Clone,
    G: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
            && self.size == other.size
            && self.item_size == other.item_size
            && self.onscroll == other.onscroll
    }
}

/// Virtualized list component.
#[allow(non_snake_case)]
pub fn List<T, F, G>(props: ListProps<F, G>) -> Element
where
    T: 'static,
    F: Clone + 'static + Fn(&T) -> Element,
    G: Clone + Lazy<Value = T> + Clone + 'static,
{
    let mut list = UseList::builder()
        .len(props.len)
        .size(props.size)
        .item_size(props.item_size)
        .use_list(props.make_value.clone());

    let values_signal = list.lazy.values();
    let values_ref = values_signal.read();
    let rows = values_ref.iter().enumerate().map(move |(idx, value)| {
        let top = (list.scroll_range.start() + idx) as f64 * *list.scroll_range.item_size.read();
        rsx!(
            div {
                key: "{top}",
                position: "absolute",
                top: "{top}px",
                left: 0,
                width: "100%",
                height: "{list.scroll_range.item_size}px",
                overflow: "hidden",
                {(props.make_item)(value)}
            }
        )
    });

    let size = *list.scroll_range.size.read();
    let inner_size = list.scroll_range.item_size * props.len as f64;
    rsx!(
        div {
            height: "{size}px",
            overflow: "scroll",
            onmounted: move |event| list.mounted.onmounted(event),
            onscroll: move |_| {
                list.scroll();
                if let Some(handler) = &props.onscroll {
                    handler.call(())
                }
            },
            div {
                position: "relative",
                height: "{inner_size}px",
                overflow: "hidden",
                {rows}
            }
        }
    )
}
