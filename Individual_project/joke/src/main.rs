use actix_web::{get, App, HttpServer, HttpResponse};
use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize)]
struct Joke {
    joke: String,
}

async fn get_joke() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = "https://icanhazdadjoke.com/";
    let joke: Joke = client.get(url)
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(joke.joke)
}

#[get("/")]
async fn index() -> HttpResponse {
    let joke = get_joke().await.unwrap_or_else(|_| "No joke found".to_string());
    println!("{}", joke);
    HttpResponse::Ok().body(format!("{}", joke))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
