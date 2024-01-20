use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::features::{
    auth::{sign_in_page::*, sign_up_page::*},
    home::home_page::*,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route
                    path="login"
                    view=move || view! { <SignInPage /> }
                />
                <Route
                    path="register"
                    view=move || view! {  <SignUpPage /> }
                />
                <Route
                    path=""
                    view=move || view! {  <HomePage /> }
                />
            </Routes>
        </Router>
    }
}
