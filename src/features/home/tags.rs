use leptos::*;

#[component]
pub fn Tags() -> impl IntoView {
    view! {
        <div class="sidebar">
            <p>"Popular Tags"</p>
            <div class="tag-list">
                <a href="" class="tag-pill tag-default">"programming"</a>
                <a href="" class="tag-pill tag-default">"javascript"</a>
            </div>
        </div>
    }
}
