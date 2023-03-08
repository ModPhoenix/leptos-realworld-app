use leptos::*;

#[component]
pub fn Banner(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="banner">
            <div class="container">
                <h1 class="logo-font">"conduit"</h1>
                <p>"A place to share your knowledge."</p>
            </div>
        </div>
    }
}
