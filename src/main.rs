mod languages;
mod templates;
mod utils;

use actix_web::{error::InternalError, get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sailfish::TemplateOnce;
use std::env;

const HEART_SVG: &str = include_str!("heart.svg");

#[get("/styles/{name:.*}")]
async fn styles(path: web::Path<String>) -> impl Responder {
    templates::render_scss(path.into_inner().as_str())
}

#[get("/")]
async fn home(page_url: web::Data<String>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (page_lang, texts) = utils::get_language_texts(&req);
    let page_url: String = page_url.into_inner().to_string();

    let page_texts: &languages::PageTexts = texts
        .pages
        .get("home")
        .expect("Cannot get page texts.");

    let template: templates::Home = templates::Home {
        page_id: "home",

        page_lang,
        page_texts: page_texts.clone(),
        texts,

        page_url: page_url.clone(),
        page_keywords: "Daniel Solarte Chaverra, Developer, ReactJS, TypeScript, JavaScript, js, programmer, software, NodeJS, Deno, Rust, Ionic, Figma, danielsolartech, 100DaysOfCode, portfolio, it, technology, service workers, pwa, ts, react",
        page_image: format!("{}assets/images/avatar.png", page_url),

        heart_svg: HEART_SVG,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?))
}

#[get("*")]
async fn error404(page_url: web::Data<String>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (page_lang, texts) = utils::get_language_texts(&req);
    let page_url: String = page_url.into_inner().to_string();

    let page_texts: &languages::PageTexts = texts
        .pages
        .get("error404")
        .expect("Cannot get page texts.");

    let template: templates::Error404 = templates::Error404 {
        page_id: "error404",

        page_lang,
        page_texts: page_texts.clone(),
        texts,

        page_url,
        page_keywords: "",
        page_image: String::new(),

        heart_svg: HEART_SVG,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?))
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
            .service(styles)
            .service(home)
            .service(error404)
    })
    .bind((host, port))?
    .run()
    .await
}
