use leptos::*;
use web_sys::window;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Login Page"</h1>
            <button on:click=move |_| {
                if let Some(win) = window() {
                    let _ = win.location().set_href("/api/auth/github");
                }
            }>"Login with GitHub"</button>
        </div>
    }
}