use leptos::*;
use leptos_router::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="navbar navbar-light">
            <div class="container">
            <A class="navbar-brand" href="/">"conduit"</A>
            <ul class="nav navbar-nav pull-xs-right">
              <li class="nav-item">
                <A class="nav-link active" href="/">"Home"</A>
              </li>
              <li class="nav-item">
                <A class="nav-link" href="/"><i class="ion-compose"></i>" New Article "</A>
              </li>
              <li class="nav-item">
                <A class="nav-link" href="/"><i class="ion-gear-a"></i>" Settings "</A>
              </li>
              <li class="nav-item">
                <A class="nav-link" href="/login">"Sign in"</A>
              </li>
              <li class="nav-item">
                <A class="nav-link" href="/register">"Sign up"</A>
              </li>
            </ul>
          </div>
        </nav>
    }
}
