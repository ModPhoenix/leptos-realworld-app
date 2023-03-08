use leptos::*;

use crate::{
    api::get_articles,
    features::home::{article_preview::*, feed_toggle::*, tags::*},
};

#[component]
pub fn Feed(cx: Scope) -> impl IntoView {
    let articles = create_local_resource(cx, move || (), |_| get_articles());

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                    .collect::<Vec<_>>()
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just implement our happy path and let the framework handle the rest
    let articles_view = move || {
        articles.with(cx, |data| {
            data.iter()
                .map(|res| res.articles.clone())
                .map(|articles| {
                    view! {
                        cx,
                        <For
                            each=move || articles.clone()
                            key=|article| article.slug.clone()
                            view=move |cx, _article | {
                                view! { cx,
                                    <ArticlePreview />
                                }
                            }
                        />
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        cx,
        <div class="container page">
            <div class="row">
                <div class="col-md-9">
                    <FeedToggle />
                    <ErrorBoundary fallback>
                        <Transition fallback=move || view! { cx, <div>"Loading (Suspense Fallback)..."</div>}>
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
