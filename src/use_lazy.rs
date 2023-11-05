use crate::Factory;
use dioxus::prelude::{to_owned, use_coroutine, Coroutine, Scope, UnboundedReceiver};
use dioxus_signals::{use_signal, Signal};
use futures::StreamExt;
use std::{cmp::Ordering, collections::VecDeque, ops::Range};

enum Message {
    Range(Range<usize>),
    Refresh,
}

pub fn use_lazy<'a, T, F>(cx: Scope<'a, T>, make_value: F) -> &'a UseLazy<F::Item>
where
    F: Factory + 'static,
{
    let values = use_signal(cx, || VecDeque::new());

    let mut last = 0..0;
    let task = use_coroutine(cx, |mut rx| async move {
        while let Some(msg) = rx.next().await {
            match msg {
                Message::Range(range) => {
                    match range.start.cmp(&last.start) {
                        Ordering::Less => {
                            let mut rows_ref = values.write();
                            let values = make_value.make(range.start..last.start, true).await;
                            for value in values.into_iter() {
                                rows_ref.push_front(value);
                            }
                        }
                        Ordering::Greater => {
                            let mut rows_ref = values.write();
                            for _ in 0..range.start - last.start {
                                rows_ref.pop_front();
                            }
                        }
                        Ordering::Equal => {}
                    }

                    if range.start != range.end {
                        match range.end.cmp(&last.end) {
                            Ordering::Greater => {
                                let mut rows_ref = values.write();
                                let values = make_value.make(last.end..range.end, false).await;
                                for value in values.into_iter() {
                                    rows_ref.push_back(value);
                                }
                            }
                            Ordering::Less => {
                                let mut rows_ref = values.write();
                                for _ in 0..last.end - range.end {
                                    rows_ref.pop_back();
                                }
                            }
                            Ordering::Equal => {}
                        }
                    }

                    last = range;
                }
                Message::Refresh => {
                    let mut rows_ref = values.write();
                    rows_ref.clear();

                    let values = make_value.make(last.clone(), false).await;
                    for value in values.into_iter() {
                        rows_ref.push_back(value);
                    }
                }
            }
        }
    });

    to_owned![task];
    cx.bump().alloc(UseLazy { task, values })
}

pub struct UseLazy<V: 'static> {
    pub values: Signal<VecDeque<V>>,
    task: Coroutine<Message>,
}

impl<V> UseLazy<V> {
    pub fn set(&self, range: Range<usize>) {
        self.task.send(Message::Range(range))
    }

    pub fn refresh(&self) {
        self.task.send(Message::Refresh)
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
