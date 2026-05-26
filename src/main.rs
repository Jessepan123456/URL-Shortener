use axum::{
    Json, Router,
    routing::{get, post},
    serve,
};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::net::TcpListener;


#[derive(Deserialize)]
struct Request {
    url: String,
}

#[tokio::main]
async fn main() {
    //Makes the website server
    let app = Router::new()
        .route("/", get(rust_page));

    //Listen for what server it makes
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running on http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}

async fn rust_page() -> String {
    let mut url_id: HashMap<String, String> = HashMap::new();
    let id = nanoid::nanoid!(6);

    url_id.insert(id.clone(), "https//youtube.com".to_string());

    match url_id.get(&id) {
        Some(url) => url.clone(),
        None => "No URL found".to_string(),
    }
}

async fn shorten(Json(body): Json<Request>) -> String {
    format!("Got URL: {}", body.url)
}

// fn main() {
//     let mut url_id: HashMap<String, String> = HashMap::new();
//     let id = nanoid::nanoid!(6);
//     println!("Nano ID: {}", id);

//     url_id.insert(id.to_string(), "https//youtube.com".to_string());

//     println!("{:?}", url_id)
// }

// --- Plan ---
//1. Send URL link
//2. Generate a ID that unique to that URL
//3. Store it with that ID
//4. Return the shortener link

//Add a clickable for that new link
//Maps to that correct URL Link
