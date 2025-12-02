use dioxus::prelude::*;

use crate::components::{add::Add, list::List};

#[component]
pub fn Home() -> Element {
    rsx!(Add {}, List {})
}
