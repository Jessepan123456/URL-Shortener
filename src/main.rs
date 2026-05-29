mod url;
mod test;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    serve,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Request {
    url: String,
}

type DB = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    //Makes the website server
    //Our Server Hashmap
    let db: DB = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/:id", get(redirect))
        .route("/shorten", post(shorten))
        .with_state(db);

    //Listen for what server it makes
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running on http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}

async fn redirect(State(db): State<DB>, Path(id): Path<String>) -> Redirect {
    let map = db.lock().unwrap();
    if let Some(link) = map.get(&id) {
        Redirect::to(link)
    } else {
        Redirect::to("http://127.0.0.1:3000")
    }
}

async fn shorten(State(db): State<DB>, Json(body): Json<Request>) -> String {
    if body.url.is_empty() {
        "Empty URL Founded".to_string()
    } else if !url::is_valid_url(&body.url) {
        "Invalid URL Link".to_string()
    } else {
        let mut id: String = String::new();
        let mut map = db.lock().unwrap();

        //New Link
        if !url::dup_url(&mut map, &mut id, &body.url) {
            id = nanoid::nanoid!(6);
            map.insert(id.clone(), body.url.clone());
        }

        format!("Short Link : http://127.0.0.1:3000/{}", id)
    }
}

// curl -X POST http://127.0.0.1:3000/shorten -H "Content-Type: application/json" -d "{\"url\":\"https://youtube.com\"}"
// curl -L http://127.0.0.1:3000/aB92xQ
