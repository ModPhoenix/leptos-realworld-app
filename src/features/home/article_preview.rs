use leptos::*;

use crate::api::Article;

#[component]
pub fn ArticlePreview(article: Article) -> impl IntoView {
    view! {
        <div class="article-preview">
            <div class="article-meta">
                <a href="profile.html">
                    <img src={article.author.image} />
                </a>
                <div class="info">
                    <a href="" class="author">{article.author.username}</a>
                    <span class="date">"January 20th"</span>
                </div>
                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                    <i class="ion-heart"></i> {article.favorites_count}
                </button>
            </div>
            <a href="" class="preview-link">
                <h1>{article.title}</h1>
                <p>{article.description}</p>
                <span>"Read more..."</span>
            </a>
        </div>
    }
}
