use crate::{use_lazy, Direction, Factory, UseLazy, UseScrollRange};
use dioxus::prelude::{to_owned, Scope};
use dioxus_use_mounted::{use_mounted, UseMounted};
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

    pub fn use_list<'a, T>(&mut self, cx: Scope<'a, T>, make_value: F) -> &'a UseList<F::Item>
    where
        F: Factory + 'static,
    {
        let mounted = use_mounted(cx);
        let lazy = use_lazy(cx, make_value);
        to_owned![lazy];

        let inner = self.inner.take().unwrap();
        let lazy_clone = lazy.clone();
        let scroll_range = UseScrollRange::builder()
            .size(inner.size)
            .item_size(inner.item_size)
            .len(inner.len)
            .use_scroll_range(cx, move |range| lazy_clone.set(range));

        cx.bump().alloc(UseList {
            mounted,
            scroll_range,
            lazy,
        })
    }
}
pub struct UseList<V: 'static> {
    pub mounted: UseMounted,
    pub scroll_range: UseScrollRange,
    pub lazy: UseLazy<V>,
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
        Self {
            mounted: self.mounted.clone(),
            scroll_range: self.scroll_range.clone(),
            lazy: self.lazy.clone(),
        }
    }
}

impl<V> PartialEq for UseList<V> {
    fn eq(&self, other: &Self) -> bool {
        self.mounted == other.mounted
            && self.scroll_range == other.scroll_range
            && self.lazy == other.lazy
    }
}
