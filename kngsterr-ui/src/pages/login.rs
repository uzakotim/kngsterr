use leptos::*;
use web_sys::window;
use gloo_console::log;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Login Page"</h1>
            <button on:click=move |_| {
                if let Some(win) = window() {
                    match win.location().set_href("/api/auth/github") {
                        Ok(_) => (), // Success, do nothing or log
                        Err(err) => {
                            // Handle the error (e.g., log it to the console)
                            log!("Error redirecting to GitHub: {:?}", err);
                            // Optionally display an error message to the user
                            // alert!("Error during GitHub login."); // Requires web_sys::console::alert
                        }
                    }
                }
            }>"Login with GitHub"</button>

            <button on:click=move |_| {
                if let Some(win) = window() {
                    match win.location().set_href("/api/auth/google") {
                        Ok(_) => (),
                        Err(err) => {
                            log!("Error redirecting to Google: {:?}", err);
                        }
                    }
                }
            }>"Login with Google"</button>
        </div>
    
    }
}