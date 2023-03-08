use leptos::*;

#[component]
pub fn FeedToggle(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="feed-toggle">
            <ul class="nav nav-pills outline-active">
                <li class="nav-item">
                    <a class="nav-link disabled" href="">"Your Feed"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" href="">"Global Feed"</a>
                </li>
            </ul>
        </div>
    }
}
