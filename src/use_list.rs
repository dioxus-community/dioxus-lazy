use dioxus::prelude::{use_effect, Scope};
use dioxus_signals::{use_signal, Signal};
use dioxus_use_mounted::{use_mounted, UseMounted};
use std::collections::VecDeque;

pub fn use_list<T, V, F>(
    cx: Scope<T>,
    len: usize,
    height: f64,
    item_height: f64,
    make_value: F,
) -> UseList<V>
where
    F: Fn(usize) -> V + Clone + 'static,
    V: 'static,
{
    let mounted = use_mounted(cx);
    let scroll = use_signal(cx, || 0);
    let values = use_signal(cx, || VecDeque::new());

    let height_signal = use_signal(cx, || height);
    use_effect(cx, &height, |_| {
        height_signal.set(height);
        async {}
    });

    let item_height_signal = use_signal(cx, || item_height);
    use_effect(cx, &item_height, |_| {
        item_height_signal.set(item_height);
        async {}
    });

    let mut last_top_row = 0;
    let mut last_bottom_row = 0;
    let make_value = make_value;
    dioxus_signals::use_effect(cx, move || {
        let item_height = *item_height_signal();
        let top_row = (*scroll() as f64 / item_height).floor() as usize;
        let total_rows = (*height_signal() / item_height).floor() as usize + 1;
        let bottom_row = (top_row + total_rows).min(len);

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

    UseList {
        mounted,
        scroll,
        values,
        height: height_signal,
        item_height: item_height_signal,
    }
}

pub struct UseList<V: 'static> {
    pub mounted: UseMounted,
    pub scroll: Signal<i32>,
    pub values: Signal<VecDeque<V>>,
    pub height: Signal<f64>,
    pub item_height: Signal<f64>,
}

impl<V> Clone for UseList<V> {
    fn clone(&self) -> Self {
        Self {
            mounted: self.mounted.clone(),
            scroll: self.scroll.clone(),
            values: self.values.clone(),
            height: self.height.clone(),
            item_height: self.item_height.clone(),
        }
    }
}

impl<V> Copy for UseList<V> {}

impl<V> PartialEq for UseList<V> {
    fn eq(&self, other: &Self) -> bool {
        self.mounted == other.mounted
            && self.scroll == other.scroll
            && self.values == other.values
            && self.height == other.height
            && self.item_height == other.item_height
    }
}
