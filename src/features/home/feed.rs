use leptos::*;

use crate::{
    api::get_articles,
    features::home::{article_preview::*, feed_toggle::*, tags::*},
};

#[component]
pub fn Feed() -> impl IntoView {
    let articles = create_local_resource(move || (), |_| get_articles());

    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! {<li>{e.to_string()}</li>})
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just use `.and_then()` to map over the happy path
    let articles_view = move || {
        articles.and_then(|data| {
            data.articles
                .iter()
                .map(|article| view! { <ArticlePreview article=article.clone() /> })
                .collect_view()
        })
    };

    view! {
        <div class="container page">
            <div class="row">
                <div class="col-md-9">
                    <FeedToggle />
                    <ErrorBoundary fallback>
                        <Transition fallback=move || view! {
                            <div class="article-preview" ng-hide="!$ctrl.loading">
                              "Loading articles..."
                            </div>
                        }>
                            {articles_view}
                        </Transition>
                    </ErrorBoundary>
                </div>
                <div class="col-md-3">
                    <Tags />
                </div>
            </div>
        </div>
    }
}
