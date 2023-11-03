# Dioxus-lazy
Virtualized components for dioxus

```rust
use dioxus::prelude::*;
use dioxus_lazy::List;

fn app(cx: Scope) -> Element {
    render!(List {
        len: 100,
        height: 400.,
        item_height: 20.,
        make_item: move |idx: &usize| render!("Item {*idx}"),
        make_value: |idx| idx
    })
}
```

```rust
fn app(cx: Scope) -> Element {
    let list = UseList::builder()
        .direction(Direction::Row)
        .size(500.)
        .use_list(cx, |idx| idx);
    
    render!(div {
        onmounted: move |event| list.mounted.onmounted(event)
    })
}
```