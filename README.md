# Dioxus-lazy

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dioxus-community/dioxus-lazy#license)
[![Crates.io](https://img.shields.io/crates/v/dioxus-lazy.svg)](https://crates.io/crates/dioxus-lazy)
[![Docs](https://docs.rs/dioxus-lazy/badge.svg)](https://docs.rs/cdk-builder/latest/dioxus-lazy)
[![CI](https://github.com/dioxus-community/dioxus-lazy/workflows/CI/badge.svg)](https://github.com/dioxus-community/dioxus-lazy/actions)


Virtualized components for dioxus

```rust
use dioxus::prelude::*;
use dioxus_lazy::{lazy, List};

fn app() -> Element {
    rsx! {
        List {
            len: 100,
            size: 400.,
            item_size: 20.,
            make_item: move |idx: &usize| rsx!("Item {*idx}"),
            make_value: lazy::from_fn(|idx| { idx })
        }

        // Or with async!

        List {
            len: 100,
            size: 400.,
            item_size: 20.,
            make_item: move |idx: &usize| rsx!("Async item {*idx}"),
            make_value: lazy::from_async_fn(|idx| async move { idx })
        }
    }
}
```

```rust
use dioxus::prelude::*;
use dioxus_lazy::{factory, Direction, UseList};

fn app() -> Element {
    let list = UseList::builder()
        .direction(Direction::Row)
        .size(500.)
        .use_list(cx, factory::from_fn(|idx| async move { idx }));

    rsx!(div {
        onmounted: move |event| list.mounted.onmounted(event)
    })
}
