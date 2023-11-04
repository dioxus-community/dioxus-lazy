use crate::Factory;
use dioxus::prelude::{to_owned, use_coroutine, Coroutine, Scope, UnboundedReceiver};
use dioxus_signals::{use_signal, Signal};
use futures::StreamExt;
use std::{cmp::Ordering, collections::VecDeque, ops::Range};

pub fn use_lazy<'a, T, F>(cx: Scope<'a, T>, make_value: F) -> &'a UseLazy<F::Item>
where
    F: Factory + 'static,
{
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
    cx.bump().alloc(UseLazy { task, values })
}

pub struct UseLazy<V: 'static> {
    pub values: Signal<VecDeque<V>>,
    task: Coroutine<(usize, usize)>,
}

impl<V> UseLazy<V> {
    pub fn set(&self, range: Range<usize>) {
        self.task.send((range.start, range.end))
    }
}

impl<V> Clone for UseLazy<V> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            task: self.task.clone(),
        }
    }
}

impl<V> PartialEq for UseLazy<V> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.task == other.task
    }
}
