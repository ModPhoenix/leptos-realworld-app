use leptos::*;

use crate::{
    components::layout::*,
    features::home::{banner::*, feed::*},
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Layout>
            <div class="home-page">
                <Banner />
                <Feed />
            </div>
        </Layout>
    }
}
