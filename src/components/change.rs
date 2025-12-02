use dioxus::prelude::*;

use crate::Item;
#[component]
pub fn Change(list_singal: Signal<Vec<Item>>, id: i64, mass: i64) -> Element {
    rsx!("toggle")
}
