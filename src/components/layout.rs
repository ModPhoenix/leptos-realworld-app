use leptos::*;

use crate::components::{footer::*, header::*};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <Header />
            {children()}
        <Footer />
    }
}
