use crate::{lazy::Values, Factory};
use dioxus_lib::prelude::*;
use futures::StreamExt;
use std::{cmp::Ordering, collections::VecDeque, ops::Range, rc::Rc};

enum Message {
    Range(Range<usize>),
    Refresh,
}

pub fn use_lazy_async<F>(make_value: F) -> UseLazyAsync<F::Item>
where
    F: Factory + 'static,
{
    let mut values = use_signal(VecDeque::new);

    let last = 0..0;
    let make_value = Rc::new(make_value);

    let task = use_coroutine(move |mut rx| {
        let mut last = last.clone();
        let make_value = make_value.clone();

        async move {
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
        }
    });

    UseLazyAsync {
        task: CopyValue::new(task),
        values,
    }
}

pub struct UseLazyAsync<V: 'static> {
    pub values: Signal<VecDeque<V>>,
    task: CopyValue<Coroutine<Message>>,
}

impl<V> Values for UseLazyAsync<V> {
    type Value = V;

    fn values(&self) -> Signal<VecDeque<Self::Value>> {
        self.values
    }

    fn set(&mut self, range: Range<usize>) {
        self.task.read().send(Message::Range(range))
    }

    fn refresh(&mut self) {
        self.task.read().send(Message::Refresh)
    }
}

impl<V> Clone for UseLazyAsync<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V> Copy for UseLazyAsync<V> {}

impl<V> PartialEq for UseLazyAsync<V> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.task == other.task
    }
}
