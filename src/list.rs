use crate::use_list::UseList;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps<F, G> {
    len: usize,
    size: f64,
    item_size: f64,
    make_item: F,
    make_value: G,
}

#[allow(non_snake_case)]
pub fn List<'a, T: 'static, F, G>(cx: Scope<'a, ListProps<F, G>>) -> Element<'a>
where
    F: Fn(&T) -> Element<'a>,
    G: Fn(usize) -> T + Clone + 'static,
{
    let list = UseList::builder()
        .len(cx.props.len)
        .size(cx.props.size)
        .item_size(cx.props.item_size)
        .use_list(cx, cx.props.make_value.clone());

    let top_row = (*list.scroll.read() as f64 / *list.item_size.read()).floor() as usize;
    let values_ref = list.values.read();
    let rows = values_ref.iter().enumerate().map(|(idx, value)| {
        render!(
            div {
                position: "absolute",
                top: "{(top_row + idx) as f64 * *list.item_size.read()}px",
                left: 0,
                width: "100%",
                height: "{list.item_size.read()}px",
                overflow: "hidden",
                (cx.props.make_item)( value)
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
                if let Some(mounted) = &*list.mounted.signal.read() {
                    let elem: &web_sys::Element = mounted
                        .get_raw_element()
                        .unwrap()
                        .downcast_ref()
                        .unwrap();
                    list.scroll.set(elem.scroll_top());
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
