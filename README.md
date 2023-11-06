# Dioxus-lazy
Virtualized components for dioxus

```rust
use dioxus::prelude::*;
use dioxus_lazy::{lazy, List};

fn app(cx: Scope) -> Element {
    render! {
        List {
            len: 100,
            size: 400.,
            item_size: 20.,
            make_item: move |idx: &usize| render!("Item {*idx}"),
            make_value: lazy::from_fn(|idx| { idx })
        }

        // Or with async!

        List {
            len: 100,
            size: 400.,
            item_size: 20.,
            make_item: move |idx: &usize| render!("Async item {*idx}"),
            make_value: lazy::from_async_fn(|idx| async move { idx })
        }
    }
}
```

```rust
use dioxus::prelude::*;
use dioxus_lazy::{factory, Direction, UseList};

fn app(cx: Scope) -> Element {
    let list = UseList::builder()
        .direction(Direction::Row)
        .size(500.)
        .use_list(cx, factory::from_fn(|idx| async move { idx }));

    render!(div {
        onmounted: move |event| list.mounted.onmounted(event)
    })
}
```
