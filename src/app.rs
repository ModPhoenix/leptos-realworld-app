use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::features::{
    auth::{sign_in_page::*, sign_up_page::*},
    home::home_page::*,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Router>
            <Routes>
                <Route
                    path="login"
                    view=move |cx| view! { cx, <SignInPage /> }
                />
                <Route
                    path="register"
                    view=move |cx| view! { cx, <SignUpPage /> }
                />
                <Route
                    path=""
                    view=move |cx| view! { cx, <HomePage /> }
                />
            </Routes>
        </Router>
    }
}
