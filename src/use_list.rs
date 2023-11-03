use dioxus::prelude::{to_owned, use_coroutine, use_effect, Scope};
use dioxus_signals::{use_signal, Signal};
use dioxus_use_mounted::{use_mounted, UseMounted};
use futures::StreamExt;
use std::{collections::VecDeque, future::Future, marker::PhantomData, ops::Range};

pub enum Direction {
    Row,
    Column,
}

struct Inner {
    direction: Direction,
    len: usize,
    size: f64,
    item_size: f64,
}

pub struct Builder<V> {
    inner: Option<Inner>,
    _marker: PhantomData<V>,
}

impl<V> Builder<V> {
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

    pub fn use_list<T, F, Fut>(&mut self, cx: Scope<T>, make_value: F) -> UseList<V>
    where
        F: Fn(usize) -> Fut + Clone + 'static,
        Fut: Future<Output = V>,
        V: 'static,
    {
        let inner = self.inner.take().unwrap();
        let size = inner.size;
        let item_size = inner.item_size;
        let len = inner.len;

        let mounted = use_mounted(cx);
        let scroll = use_signal(cx, || 0);
        let values = use_signal(cx, || VecDeque::new());

        let size_signal = use_signal(cx, || size);
        use_effect(cx, &size, |_| {
            size_signal.set(size);
            async {}
        });

        let item_size_signal = use_signal(cx, || item_size);
        use_effect(cx, &item_size, |_| {
            item_size_signal.set(item_size);
            async {}
        });

        let mut last_top_row = 0;
        let mut last_bottom_row = 0;
        let task = use_coroutine(cx, |mut rx| async move {
            while let Some((top_row, bottom_row)) = rx.next().await {
                if top_row < last_top_row {
                    let mut rows_ref = values.write();
                    for idx in (top_row..last_top_row).rev() {
                        let value = make_value(idx).await;
                        rows_ref.push_front(value);
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
                            let value = make_value(idx).await;
                            rows_ref.push_back(value);
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
            }
        });

        to_owned![task];
        dioxus_signals::use_effect(cx, move || {
            let item_height = *item_size_signal();
            let top_row = (*scroll() as f64 / item_height).floor() as usize;
            let total_rows = (*size_signal() / item_height).floor() as usize + 1;
            let bottom_row = (top_row + total_rows).min(len);
            task.send((top_row, bottom_row))
        });

        UseList {
            mounted,
            scroll,
            values,
            size: size_signal,
            item_size: item_size_signal,
            len,
        }
    }
}

pub struct UseList<V: 'static> {
    pub mounted: UseMounted,
    pub scroll: Signal<i32>,
    pub values: Signal<VecDeque<V>>,
    pub size: Signal<f64>,
    pub item_size: Signal<f64>,
    pub len: usize,
}

impl<V> UseList<V> {
    pub fn builder() -> Builder<V> {
        Builder {
            inner: Some(Inner {
                direction: Direction::Row,
                len: 0,
                size: 400.,
                item_size: 20.,
            }),
            _marker: PhantomData,
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

impl<V> Clone for UseList<V> {
    fn clone(&self) -> Self {
        Self {
            mounted: self.mounted.clone(),
            scroll: self.scroll.clone(),
            values: self.values.clone(),
            size: self.size.clone(),
            item_size: self.item_size.clone(),
            len: self.len,
        }
    }
}

impl<V> Copy for UseList<V> {}

impl<V> PartialEq for UseList<V> {
    fn eq(&self, other: &Self) -> bool {
        self.mounted == other.mounted
            && self.scroll == other.scroll
            && self.values == other.values
            && self.size == other.size
            && self.item_size == other.item_size
    }
}
