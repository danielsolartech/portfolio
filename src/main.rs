mod languages;
mod templates;
mod utils;

use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/styles/{name:.*}")]
async fn styles(path: web::Path<String>) -> impl Responder {
    templates::render_scss(path.into_inner().as_str())
}

#[get("/")]
async fn about(page_url: web::Data<String>, req: HttpRequest) -> impl Responder {
    let page_lang: String = utils::get_language(&req);
    let page_url: String = page_url.into_inner().to_string();

    let lang_texts: languages::Language =
        languages::get_langague_or(&page_lang, "en").expect("Cannot parse language.");
    let page_texts: &languages::PageTexts = lang_texts
        .pages
        .get("about")
        .expect("Cannot get page texts.");

    let mut about_data: templates::TemplateData = templates::TemplateData::new();
    about_data.push(("profession", &lang_texts.about.profession));
    about_data.push(("about_me_title", &lang_texts.about.about_me_title));
    about_data.push(("about_message_1", &lang_texts.about.messages[0]));
    about_data.push(("about_message_2", &lang_texts.about.messages[1]));
    about_data.push(("about_message_3", &lang_texts.about.messages[2]));
    about_data.push(("skills_title", &lang_texts.about.skills_title));

    templates::render_html(
        "about",
        templates::HeaderData {
            page_id: "about",

            page_lang,
            page_url: page_url.clone(),

            page_title: page_texts.title.clone(),
            page_description: page_texts.description.clone(),
            page_image: format!("{}assets/images/avatar.png", page_url),

            header_texts: lang_texts.header,

            page_keywords: "Daniel Solarte Chaverra, Developer, ReactJS, TypeScript, JavaScript, js, programmer, software, NodeJS, Deno, Rust, Ionic, Figma, danielsolartech, 100DaysOfCode, portfolio, it, technology, service workers, pwa, ts, react",
        },
        lang_texts.footer,
        about_data,
    )
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
            .service(actix_files::Files::new("/assets/", "public/").show_files_listing())
            .service(styles)
            .service(about)
    })
    .bind((host, port))?
    .run()
    .await
}
