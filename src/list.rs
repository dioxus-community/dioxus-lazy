use dioxus::prelude::*;
use dioxus_signals::use_signal;
use dioxus_use_mounted::use_mounted;
use std::collections::VecDeque;

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
    let mounted = use_mounted(cx);
    let scroll = use_signal(cx, || 0);
    let values = use_signal(cx, || VecDeque::new());

    let height = use_signal(cx, || cx.props.height);
    use_effect(cx, &cx.props.height, |_| {
        height.set(cx.props.height);
        async {}
    });

    let item_height = use_signal(cx, || cx.props.item_height);
    use_effect(cx, &cx.props.item_height, |_| {
        item_height.set(cx.props.item_height);
        async {}
    });

    let mut last_top_row = 0;
    let mut last_bottom_row = 0;
    let make_value = cx.props.make_value.clone();
    dioxus_signals::use_effect(cx, move || {
        let item_height = *item_height();
        let top_row = (*scroll() as f64 / item_height).floor() as usize;
        let total_rows = (*height() / item_height).floor() as usize + 1;
        let bottom_row = top_row + total_rows;

        if top_row < last_top_row {
            let mut rows_ref = values.write();
            for idx in (top_row..last_top_row).rev() {
                rows_ref.push_front(make_value(idx));
            }
        } else if top_row > last_top_row {
            let mut rows_ref = values.write();
            for _ in 0..top_row - last_top_row {
                rows_ref.pop_front();
            }
        }

        if top_row != bottom_row {
            if bottom_row > last_bottom_row {
                let mut rows_ref = values.write();
                for idx in last_bottom_row..bottom_row {
                    rows_ref.push_back(make_value(idx));
                }
            } else if bottom_row < last_bottom_row {
                let mut rows_ref = values.write();
                for _ in 0..last_bottom_row - bottom_row {
                    rows_ref.pop_back();
                }
            }
        }

        last_top_row = top_row;
        last_bottom_row = bottom_row;
    });

    let top_row = (*scroll() as f64 / *item_height()).floor() as usize;
    let values_ref = values();
    let rows = values_ref.iter().enumerate().map(|(idx, value)| {
        render!(
            div {
                position: "absolute",
                top: "{(top_row + idx) as f64 * *item_height()}px",
                left: 0,
                width: "100%",
                height: "{item_height}px",
                overflow: "hidden",
                (cx.props.make_item)( value)
            }
        )
    });

    render!(
        div {
            height: "{height}px",
            overflow: "scroll",
            onmounted: move |event| mounted.onmounted(event),
            onscroll: move |_| {
                if let Some(mounted) = &*mounted.signal.read() {
                    let elem: &web_sys::Element = mounted
                        .get_raw_element()
                        .unwrap()
                        .downcast_ref()
                        .unwrap();
                    scroll.set(elem.scroll_top());
                }
            },
            div {
                position: "relative",
                height: "{item_height * cx.props.len as f64}px",
                overflow: "hidden",
                rows
            }
        }
    )
}
