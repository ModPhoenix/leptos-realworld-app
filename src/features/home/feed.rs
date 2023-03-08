use leptos::*;

use crate::features::home::{article_preview::*, feed_toggle::*, tags::*};

#[component]
pub fn Feed(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="container page">
            <div class="row">
                <div class="col-md-9">
                    <FeedToggle />
                    <ArticlePreview />
                    <ArticlePreview />
                </div>
                <div class="col-md-3">
                    <Tags />
                </div>
            </div>
        </div>
    }
}
