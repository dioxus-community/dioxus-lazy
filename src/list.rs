use crate::use_list::use_list;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps<F, G> {
    len: usize,
    height: f64,
    item_height: f64,
    make_item: F,
    make_value: G,
}

#[allow(non_snake_case)]
pub fn List<'a, T: 'static, F, G>(cx: Scope<'a, ListProps<F, G>>) -> Element<'a>
where
    F: Fn(&T) -> Element<'a>,
    G: Fn(usize) -> T + Clone + 'static,
{
    let list = use_list(
        cx,
        cx.props.len,
        cx.props.height,
        cx.props.item_height,
        cx.props.make_value.clone(),
    );

    let top_row = (*list.scroll.read() as f64 / *list.item_height.read()).floor() as usize;
    let values_ref = list.values.read();
    let rows = values_ref.iter().enumerate().map(|(idx, value)| {
        render!(
            div {
                position: "absolute",
                top: "{(top_row + idx) as f64 * *list.item_height.read()}px",
                left: 0,
                width: "100%",
                height: "{list.item_height.read()}px",
                overflow: "hidden",
                (cx.props.make_item)( value)
            }
        )
    });

    let height = *list.height.read();
    render!(
        div {
            height: "{height}px",
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
                height: "{list.item_height * cx.props.len as f64}px",
                overflow: "hidden",
                rows
            }
        }
    )
}
