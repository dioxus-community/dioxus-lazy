use crate::{factory, use_lazy, use_lazy_async, UseLazy, UseLazyAsync};

use dioxus_lib::signals::Signal;
use futures::Future;
use std::{collections::VecDeque, ops::Range};

pub trait Values: Clone {
    type Value;

    fn values(&self) -> Signal<VecDeque<Self::Value>>;

    fn set(&mut self, range: Range<usize>);

    fn refresh(&mut self);
}

pub trait Lazy {
    type Value;
    type Values: Values<Value = Self::Value>;

    fn values(self) -> Self::Values;
}

pub fn from_fn<F, V>(f: F) -> FromFn<F>
where
    F: FnMut(usize) -> V + 'static,
    V: 'static,
{
    FromFn { f }
}

#[derive(Clone)]
pub struct FromFn<F> {
    f: F,
}

impl<F, V> Lazy for FromFn<F>
where
    F: FnMut(usize) -> V + 'static,
    V: 'static,
{
    type Value = V;
    type Values = UseLazy<Box<dyn FnMut(Range<usize>, bool) -> std::vec::IntoIter<V>>, V>;

    fn values(mut self) -> Self::Values {
        use_lazy(Box::new(move |range, is_rev| {
            let mut values = Vec::new();
            if is_rev {
                for idx in range.rev() {
                    values.push((self.f)(idx))
                }
            } else {
                for idx in range {
                    values.push((self.f)(idx))
                }
            }

            values.into_iter()
        }))
    }
}

pub struct FromRangeFn<F> {
    f: F,
}

impl<F, I, V> Lazy for FromRangeFn<F>
where
    F: FnMut(Range<usize>, bool) -> I + 'static,
    I: IntoIterator<Item = V>,
    V: 'static,
{
    type Value = V;
    type Values = UseLazy<F, V>;

    fn values(self) -> Self::Values {
        use_lazy(self.f)
    }
}

pub fn from_async_fn<F, Fut, V>(f: F) -> FromAsyncFn<F>
where
    F: Fn(usize) -> Fut + Clone + 'static,
    Fut: Future<Output = V> + 'static,
    V: 'static,
{
    FromAsyncFn { f }
}

#[derive(Clone, Copy)]
pub struct FromAsyncFn<F> {
    f: F,
}

impl<F, Fut, V> Lazy for FromAsyncFn<F>
where
    F: Fn(usize) -> Fut + Clone + 'static,
    Fut: Future<Output = V> + 'static,
    V: 'static,
{
    type Value = V;
    type Values = UseLazyAsync<V>;

    fn values(self) -> Self::Values {
        use_lazy_async(factory::from_fn(self.f))
    }
}

pub fn from_async_range_fn<F, Fut, I, V>(f: F) -> FromAsyncRangeFn<F>
where
    F: Fn(Range<usize>, bool) -> Fut + Clone + 'static,
    Fut: Future<Output = I> + 'static,
    I: IntoIterator<Item = V>,
    V: 'static,
{
    FromAsyncRangeFn { f }
}

#[derive(Clone, Copy)]
pub struct FromAsyncRangeFn<F> {
    f: F,
}

impl<F, Fut, I, V> Lazy for FromAsyncRangeFn<F>
where
    F: Fn(Range<usize>, bool) -> Fut + Clone + 'static,
    Fut: Future<Output = I> + 'static,
    I: IntoIterator<Item = V>,
    V: 'static,
{
    type Value = V;
    type Values = UseLazyAsync<V>;

    fn values(self) -> Self::Values {
        use_lazy_async(factory::from_range_fn(self.f))
    }
}
