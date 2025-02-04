use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Welcome to Kngsterr"</h1>
            <a href="/login">"Login"</a>
        </div>
    }
}