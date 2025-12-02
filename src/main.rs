use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod backend;
mod components;

use components::{home::Home, item::Itemm, nav::Nav};

use crate::backend::server_funktions::get_item_list;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut list = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_item_list().await {
            Ok(items) => list.set(items),
            Err(_) => (),
        }
    });

    use_context_provider(|| list);

    rsx!(
        document::Stylesheet{href: asset!("/assets/main.css")}
        document::Stylesheet{href: asset!("/assets/tailwind.css")}
        "test"
        Router::<Route> {}
    )
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/item/:id")]
    Itemm {
        list_signal: Signal<Vec<Item>>,
        id: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub mass: i64,
    pub unit: String,
    pub experation: String,
    //pub experation: Date<Local>,
}
