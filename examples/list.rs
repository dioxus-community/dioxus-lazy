use dioxus::prelude::*;
use dioxus_lazy::List;
use log::LevelFilter;

fn app(cx: Scope) -> Element {
    render!(List {
        len: 100,
        size: 400.,
        item_size: 20.,
        make_item: move |idx: &usize| render!("Item {*idx}"),
        make_value: |idx| async move { idx }
    })
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).unwrap();
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}
