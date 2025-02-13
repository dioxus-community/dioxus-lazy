use crate::{
    lazy::{Lazy, Values},
    Direction, UseScrollRange,
};
use dioxus_lib::prelude::*;
use dioxus_use_mounted::{use_mounted, UseMounted};
use dioxus_web::WebEventExt;
use std::marker::PhantomData;

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

    pub fn use_list(&mut self, make_value: F) -> UseList<F::Values>
    where
        F: Lazy,
    {
        let mounted = use_mounted();
        let lazy = make_value.values();

        let inner = self.inner.take().unwrap();
        let mut lazy_clone = lazy.clone();
        let scroll_range = UseScrollRange::builder()
            .size(inner.size)
            .item_size(inner.item_size)
            .len(inner.len)
            .use_scroll_range(move |range| lazy_clone.set(range));

        UseList {
            mounted,
            scroll_range,
            lazy,
        }
    }
}

pub struct UseList<T: 'static> {
    pub mounted: UseMounted,
    pub scroll_range: UseScrollRange,
    pub lazy: T,
}

impl<T> UseList<T> {
    pub fn builder() -> Builder<T> {
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

    pub fn scroll(&mut self) {
        if let Some(mounted) = self.mounted.signal.read().as_deref() {
            let elem = mounted.as_web_event();
            self.scroll_range.scroll.set(elem.scroll_top());
        }
    }
}

impl<T: Clone> Clone for UseList<T> {
    fn clone(&self) -> Self {
        Self {
            mounted: self.mounted,
            scroll_range: self.scroll_range,
            lazy: self.lazy.clone(),
        }
    }
}

impl<T: Copy> Copy for UseList<T> {}

impl<T: PartialEq> PartialEq for UseList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.mounted == other.mounted
            && self.scroll_range == other.scroll_range
            && self.lazy == other.lazy
    }
}
