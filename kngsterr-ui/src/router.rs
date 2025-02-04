use leptos::*;
use leptos_router::*;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=move || view! { <HomePage/> }/>
                <Route path="/login" view=move || view! { <LoginPage/> }/>
            </Routes>
        </Router>
    }
}