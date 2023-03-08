use leptos::*;

use crate::components::{footer::*, header::*};

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {
        cx,
        <Header />
            {children(cx)}
        <Footer />
    }
}
