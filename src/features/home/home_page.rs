use leptos::*;

use crate::{
    components::layout::*,
    features::home::{banner::*, feed::*},
};

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Layout>
            <div class="home-page">
                <Banner />
                <Feed />
            </div>
        </Layout>
    }
}
