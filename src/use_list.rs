use crate::Factory;
use dioxus::prelude::{to_owned, use_coroutine, use_effect, Scope, UnboundedReceiver};
use dioxus_signals::{use_signal, Signal};
use dioxus_use_mounted::{use_mounted, UseMounted};
use futures::StreamExt;
use std::{cmp::Ordering, collections::VecDeque, marker::PhantomData, ops::Range};

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

pub struct Builder<F> {
    inner: Option<Inner>,
    _marker: PhantomData<F>,
}

impl<F> Builder<F> {
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

    pub fn use_list<T>(&mut self, cx: Scope<T>, make_value: F) -> UseList<F::Item>
    where
        F: Factory + 'static,
    {
        let mounted = use_mounted(cx);
        let scroll = use_signal(cx, || 0);
        let values = use_signal(cx, || VecDeque::new());

        let inner = self.inner.take().unwrap();
        let len = inner.len;
        let size = use_effect_signal(cx, inner.size);
        let item_size = use_effect_signal(cx, inner.item_size);

        let mut last_top_row = 0;
        let mut last_bottom_row = 0;
        let task = use_coroutine(cx, |mut rx: UnboundedReceiver<(usize, usize)>| async move {
            while let Some((top_row, bottom_row)) = rx.next().await {
                match top_row.cmp(&last_top_row) {
                    Ordering::Less => {
                        let mut rows_ref = values.write();
                        let values = make_value.make(top_row..last_top_row, true).await;
                        for value in values.into_iter() {
                            rows_ref.push_front(value);
                        }
                    }
                    Ordering::Greater => {
                        let mut rows_ref = values.write();
                        for _ in 0..top_row - last_top_row {
                            rows_ref.pop_front();
                        }
                    }
                    Ordering::Equal => {}
                }

                if top_row != bottom_row {
                    match bottom_row.cmp(&last_bottom_row) {
                        Ordering::Greater => {
                            let mut rows_ref = values.write();
                            let values = make_value.make(last_bottom_row..bottom_row, false).await;
                            for value in values.into_iter() {
                                rows_ref.push_back(value);
                            }
                        }
                        Ordering::Less => {
                            let mut rows_ref = values.write();
                            for _ in 0..last_bottom_row - bottom_row {
                                rows_ref.pop_back();
                            }
                        }
                        Ordering::Equal => {}
                    }
                }

                last_top_row = top_row;
                last_bottom_row = bottom_row;
            }
        });

        to_owned![task];
        dioxus_signals::use_effect(cx, move || {
            let item_height = *item_size();
            let top_row = (*scroll() as f64 / item_height).floor() as usize;
            let total_rows = (*size() / item_height).floor() as usize + 1;
            let bottom_row = (top_row + total_rows).min(len);
            task.send((top_row, bottom_row))
        });

        UseList {
            mounted,
            scroll,
            values,
            size,
            item_size,
            len,
        }
    }
}

fn use_effect_signal<T, V>(cx: Scope<T>, value: V) -> Signal<V>
where
    V: PartialEq + Clone + 'static,
{
    let signal = use_signal(cx, || value.clone());
    use_effect(cx, &value, |val| {
        signal.set(val);
        async {}
    });
    signal
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
        *self
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
