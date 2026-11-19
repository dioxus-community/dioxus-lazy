use dioxus::prelude::*;
use std::rc::Rc;

pub fn use_mounted() -> UseMounted {
    let signal = use_signal(|| None);
    UseMounted { signal }
}

#[derive(Clone, Copy, PartialEq)]
pub struct UseMounted {
    pub signal: Signal<Option<Rc<MountedData>>>,
}

impl UseMounted {
    pub fn onmounted(self, event: Event<MountedData>) {
        self.mount(event.data());
    }

    pub fn mount(mut self, data: Rc<MountedData>) {
        self.signal.set(Some(data));
    }
}
