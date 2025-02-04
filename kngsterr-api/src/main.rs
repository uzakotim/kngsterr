use axum::{routing::get, Router, response::Redirect};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
    // let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID not set");
    let cors = CorsLayer::new()
    .allow_methods(Any)  // Allows all HTTP methods
    .allow_headers(Any)  // Allows all headers
    .allow_origin(Any);  // Allows requests from frontend

    let app = Router::new()
        .route("/api/auth/google", get(move || async move {
            let redirect_url = format!(
                "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri=http://localhost:8000/callback&response_type=code&scope=email profile",
                google_client_id
            );
            // format!("Redirect to: {}", redirect_url)
            Redirect::temporary(&redirect_url)
        }))
        // .route("localhost:3000/api/auth/github", get(move || async move {
        //     let redirect_url = format!(
        //         "https://github.com/login/oauth/authorize?client_id={}&redirect_uri=http://localhost:8000/callback&scope=user",
        //         github_client_id
        //     );
        //     format!("Redirect to: {}", redirect_url)
        // }))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
