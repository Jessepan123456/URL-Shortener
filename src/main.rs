mod files;
mod test;
mod url;

use axum::{
    Form, Router,
    extract::{Path, State},
    response::{Html, Redirect},
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

    //Load
    let db: DB = Arc::new(Mutex::new(files::load()));

    let app = Router::new()
        .route("/", get(home))
        .route("/:id", get(redirect))
        .route("/shorten", post(shorten))
        .with_state(db);

    //Listen for what server it makes
    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to Listen: {}", e);
            return;
        }
    };

    println!("Server running on http://127.0.0.1:3000");

    if let Err(e) = serve(listener, app).await {
        println!("Failed to Start the Server: {}", e)
    }
}

async fn home() -> Html<&'static str> {
    Html(
        r#"
        <h1 style="text-align: center;">URL Shortener</h1>

        <form action="/shorten" method ="post" style="text-align: center;">
            <input type="text" name="url" placeholder="Enter URL">
            <button type="submit">Shorten</button>
        </form>
    "#,
    )
}

//Redirect to the Link for terminal
async fn redirect(State(db): State<DB>, Path(id): Path<String>) -> Redirect {
    let map = db.lock().unwrap();
    if let Some(link) = map.get(&id) {
        Redirect::to(link)
    } else {
        Redirect::to("http://127.0.0.1:3000")
    }
}

//Shorten the link
async fn shorten(State(db): State<DB>, Form(body): Form<Request>) -> Html<String> {
    if body.url.is_empty() {
        Html(format!(
            r#"
            <h1 style="text-align: center;">Invalid URL Link</h1>
        "#
        ))
    } else if !url::is_valid_url(&body.url) {
        Html(format!(
            r#"
            <h1 style="text-align: center;">Invalid URL Link</h1>
        "#
        ))
    } else {
        let mut id: String = String::new();
        let mut map = db.lock().unwrap();
        //New Link
        if !url::dup_url(&mut map, &mut id, &body.url) {
            id = nanoid::nanoid!(6);
            map.insert(id.clone(), body.url.clone());

            //Save
            files::save(&*map);
        }

        Html(format!(
            r#"
            <h1 style="text-align: center;">Shortened URL</h1>

            <a href="http://127.0.0.1:3000/{0}"
                style="display: block; text-align: center;">
                http://127.0.0.1:3000/{0}
            </a>
        "#,
            id
        ))
    }
}

// curl -X POST http://127.0.0.1:3000/shorten -H "Content-Type: application/json" -d "{\"url\":\"https://youtube.com\"}"
// curl -L http://127.0.0.1:3000/aB92xQ
