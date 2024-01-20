use leptos::*;

pub use app::*;

mod api;
mod app;
mod components;
mod features;

fn main() {
    mount_to_body(|| view! {<App/>})
}
