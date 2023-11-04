use dioxus::prelude::*;
use dioxus_lazy::{factory, Direction, UseList};
use log::LevelFilter;

fn app(cx: Scope) -> Element {
    let list = UseList::builder()
        .direction(Direction::Row)
        .size(500.)
        .use_list(cx, factory::from_fn(|idx| async move { idx }));

    render!(div {
        onmounted: move |event| list.mounted.onmounted(event)
    })
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).unwrap();
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}
