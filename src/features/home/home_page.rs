use leptos::*;

use crate::components::layout::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Layout>"Home Page"</Layout>
    }
}
