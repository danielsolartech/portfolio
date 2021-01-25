mod data;
mod languages;
mod templates;
mod utils;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn home(page_url: web::Data<String>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    templates::render("home", page_url, req)
}

#[get("/projects")]
async fn projects(page_url: web::Data<String>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    templates::render("projects", page_url, req)
}

#[get("*")]
async fn error404(page_url: web::Data<String>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    templates::render("error404", page_url, req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read .env file.
    dotenv().ok();

    // Get `API_HOST` from the environment variables.
    let host: String = env::var("API_HOST").unwrap_or(String::from("127.0.0.1"));

    // Get `API_PORT` from the environment variables.
    let port: u16 = env::var("API_PORT")
        .unwrap_or(String::from("5000"))
        .parse()
        .expect("The `API_PORT` is not a valid port number.");

    // Get `API_URL` from the environment variables.
    let page_url: String = env::var("API_URL").unwrap_or(format!("http://{}:{}/", host, port));

    // Initialize the HTTP Server
    HttpServer::new(move || {
        App::new()
            .data(page_url.clone())
            .service(actix_files::Files::new("/assets/", "assets/public/").show_files_listing())
            .service(home)
            .service(projects)
            .service(error404)
    })
    .bind((host, port))?
    .run()
    .await
}
