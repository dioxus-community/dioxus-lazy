use dioxus_lazy::{lazy, List};
use dioxus_lib::prelude::*;
use dioxus_logger::tracing::Level;

fn app() -> Element {
    rsx! {
        List {
            len: 100,
            size: 400.,
            item_size: 20.,
            make_item: move |idx: &usize| rsx! { "Async item {*idx}" },
            make_value: lazy::from_async_fn(|idx| async move { idx })
        }
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).unwrap();
    console_error_panic_hook::set_once();

    dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new())
}
