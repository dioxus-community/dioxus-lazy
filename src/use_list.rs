use crate::{Direction, Factory, UseScrollRange};
use dioxus::prelude::{to_owned, use_coroutine, Scope, UnboundedReceiver};
use dioxus_signals::{use_signal, Signal};
use dioxus_use_mounted::{use_mounted, UseMounted};
use futures::StreamExt;
use std::{cmp::Ordering, collections::VecDeque, marker::PhantomData};

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
        let values = use_signal(cx, || VecDeque::new());

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
        let inner = self.inner.take().unwrap();
        let scroll_range = UseScrollRange::builder()
            .size(inner.size)
            .item_size(inner.item_size)
            .len(inner.len)
            .use_scroll_range(cx, 0, move |range| task.send((range.start, range.end)));

        UseList {
            mounted,
            scroll_range,
            values,
        }
    }
}
pub struct UseList<V: 'static> {
    pub mounted: UseMounted,
    pub scroll_range: UseScrollRange,
    pub values: Signal<VecDeque<V>>,
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

    pub fn scroll(&self) {
        if let Some(mounted) = &*self.mounted.signal.read() {
            let elem: &web_sys::Element =
                mounted.get_raw_element().unwrap().downcast_ref().unwrap();
            self.scroll_range.scroll.set(elem.scroll_top());
        }
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
            && self.scroll_range == other.scroll_range
            && self.values == other.values
    }
}
