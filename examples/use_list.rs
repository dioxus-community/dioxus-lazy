use dioxus_lazy::{lazy, Direction, UseList};
use dioxus_lib::prelude::*;
use dioxus_logger::tracing::Level;

fn app() -> Element {
    let list = UseList::builder()
        .direction(Direction::Row)
        .size(500.)
        .use_list(lazy::from_async_fn(|idx| async move { idx }));

    rsx!(div {
        onmounted: move |event| list.mounted.onmounted(event)
    })
}

fn main() {
    dioxus_logger::init(Level::INFO).unwrap();
    console_error_panic_hook::set_once();
    dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new())
}
