use crate::Direction;
use dioxus_lib::prelude::*;
use std::ops::Range;

struct Inner {
    direction: Direction,
    len: usize,
    size: f64,
    item_size: f64,
}

pub struct Builder {
    inner: Option<Inner>,
}

impl Builder {
    pub fn direction(&mut self, direction: Direction) -> &mut Self {
        self.inner.as_mut().unwrap().direction = direction;
        self
    }

    pub fn len(&mut self, len: usize) -> &mut Self {
        self.inner.as_mut().unwrap().len = len;
        self
    }

    pub fn size(&mut self, size: f64) -> &mut Self {
        self.inner.as_mut().unwrap().size = size;
        self
    }

    pub fn item_size(&mut self, item_size: f64) -> &mut Self {
        self.inner.as_mut().unwrap().item_size = item_size;
        self
    }

    pub fn use_scroll_range(
        &mut self,
        mut onscroll: impl FnMut(Range<usize>) + 'static,
    ) -> UseScrollRange {
        let inner = self.inner.take().unwrap();
        let len = inner.len;
        let size = use_effect_signal(inner.size);
        let item_size = use_effect_signal(inner.item_size);
        let scroll = use_signal(|| 0);

        use_effect(move || {
            let item_height = item_size();
            let top_row = (scroll() as f64 / item_height).floor() as usize;
            let total_rows = (size() / item_height).floor() as usize + 1;
            let bottom_row = (top_row + total_rows).min(len);
            onscroll(top_row..bottom_row)
        });

        UseScrollRange {
            scroll,
            size,
            item_size,
            len,
        }
    }
}

fn use_effect_signal<V>(value: V) -> Signal<V>
where
    V: PartialEq + Clone + 'static,
{
    let mut signal = use_signal(|| value.clone());
    use_effect(use_reactive(&value, move |val| {
        signal.set(val);
    }));
    signal
}

#[derive(Clone, Copy, PartialEq)]
pub struct UseScrollRange {
    pub scroll: Signal<i32>,
    pub size: Signal<f64>,
    pub item_size: Signal<f64>,
    pub len: usize,
}

impl UseScrollRange {
    pub fn builder() -> Builder {
        Builder {
            inner: Some(Inner {
                direction: Direction::Row,
                len: 0,
                size: 400.,
                item_size: 20.,
            }),
        }
    }

    /// Get the current start index.
    pub fn start(&self) -> usize {
        (*self.scroll.read() as f64 / *self.item_size.read()).floor() as usize
    }

    /// Get the current range of item indices.
    pub fn range(&self) -> Range<usize> {
        let start = self.start();
        let total = (*self.size.read() / *self.item_size.read()).floor() as usize + 1;
        let end = (start + total).min(self.len);
        start..end
    }
}
