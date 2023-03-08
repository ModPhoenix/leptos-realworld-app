use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <footer>
            <div class="container">
                <a href="/" class="logo-font">"conduit"</a>
                <span class="attribution">
                "An interactive learning project from "
                <a href="https://thinkster.io">"Thinkster"</a>
                ". Code design licensed under MIT."
                </span>
            </div>
        </footer>
    }
}
