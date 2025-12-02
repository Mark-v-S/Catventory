use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Nav() -> Element {
    rsx!(
        div { class: "my-5 text-purple-500 text-center text-5xl",
            Link {to: Route::Home {  }, "Inventur Liste"}
        }
        Outlet::<Route> {}
    )
}
