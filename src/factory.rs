use futures::Future;
use std::{ops::Range, pin::Pin};

pub trait Factory {
    type Item;
    type Output: IntoIterator<Item = Self::Item>;
    type Future: Future<Output = Self::Output>;

    fn make(&self, range: Range<usize>, is_rev: bool) -> Self::Future;
}

pub fn from_fn<F, Fut, V>(f: F) -> FromFn<F>
where
    F: Fn(usize) -> Fut + Clone + 'static,
    Fut: Future<Output = V> + 'static,
    V: 'static,
{
    FromFn { f }
}

#[derive(Clone, Copy)]
pub struct FromFn<F> {
    f: F,
}

impl<F, Fut, V> Factory for FromFn<F>
where
    F: Fn(usize) -> Fut + Clone + 'static,
    Fut: Future<Output = V> + 'static,
    V: 'static,
{
    type Item = V;
    type Output = std::vec::IntoIter<V>;
    type Future = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn make(&self, range: Range<usize>, is_rev: bool) -> Self::Future {
        let f = self.f.clone();
        Box::pin(async move {
            let mut values = Vec::new();

            if is_rev {
                for idx in range.rev() {
                    values.push(f(idx).await)
                }
            } else {
                for idx in range {
                    values.push(f(idx).await)
                }
            }

            values.into_iter()
        })
    }
}

pub fn from_range_fn<F, Fut, I, V>(f: F) -> FromRangeFn<F>
where
    F: Fn(Range<usize>, bool) -> Fut + Clone + 'static,
    Fut: Future<Output = I> + 'static,
    I: IntoIterator<Item = V>,
    V: 'static,
{
    FromRangeFn { f }
}

#[derive(Clone, Copy)]
pub struct FromRangeFn<F> {
    f: F,
}

impl<F, Fut, I, V> Factory for FromRangeFn<F>
where
    F: Fn(Range<usize>, bool) -> Fut + Clone + 'static,
    Fut: Future<Output = I> + 'static,
    I: IntoIterator<Item = V>,
    V: 'static,
{
    type Item = V;
    type Output = I;
    type Future = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn make(&self, input: Range<usize>, is_rev: bool) -> Self::Future {
        Box::pin((self.f)(input, is_rev))
    }
}
