use dioxus_lib::prelude::*;

use crate::lazy::Values;

use std::{cmp::Ordering, collections::VecDeque, ops::Range};

pub fn use_lazy<F, V, I>(make_value: F) -> UseLazy<F, V>
where
    F: FnMut(Range<usize>, bool) -> I + 'static,
    I: IntoIterator<Item = V>,
{
    let values = use_signal(|| VecDeque::new());
    let range = use_signal(|| 0..0);

    UseLazy {
        make_value: CopyValue::new(make_value),
        values,
        range,
    }
}

pub struct UseLazy<F: 'static, V: 'static> {
    pub values: Signal<VecDeque<V>>,
    make_value: CopyValue<F>,
    range: Signal<Range<usize>>,
}

impl<F, V, I> Values for UseLazy<F, V>
where
    F: FnMut(Range<usize>, bool) -> I + 'static,
    I: IntoIterator<Item = V>,
{
    type Value = V;

    fn values(&self) -> Signal<VecDeque<Self::Value>> {
        self.values
    }

    fn set(&mut self, range: Range<usize>) {
        let mut last = self.range.write();
        let mut values = self.values;

        match range.start.cmp(&last.start) {
            Ordering::Less => {
                let mut rows_ref = values.write();
                let values = (self.make_value).write()(range.start..last.start, true);
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
                    let values = (self.make_value).write()(last.end..range.end, false);
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

        *last = range;
    }

    fn refresh(&mut self) {
        let last = self.range.read();
        let mut values_ref = self.values.write();
        values_ref.clear();

        let values = (self.make_value).write()(last.start..last.end, false);
        for value in values.into_iter() {
            values_ref.push_back(value);
        }
    }
}

impl<F, V> Clone for UseLazy<F, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<F, V> Copy for UseLazy<F, V> {}

impl<F, V> PartialEq for UseLazy<F, V> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}
