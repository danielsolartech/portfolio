use crate::languages::{get_langague_or, Language};
use actix_web::HttpRequest;
use std::env;

pub fn get_current_directory() -> String {
    if let Ok(current_directory) = env::current_dir() {
        let mut current_directory: String = current_directory.to_str().unwrap_or("").to_string();

        if !current_directory.ends_with("/") {
            current_directory.push_str("/");
        }

        current_directory
    } else {
        String::new()
    }
}

pub fn to_url(text: &String) -> String {
    text.trim()
        .replace("/", "")
        .replace("(", "")
        .replace(")", "")
        .replace("{", "")
        .replace("¿", "")
        .replace("?", "")
        .replace(".", "")
        .replace(" ", "-")
        .replace("ñ", "n")
        .to_lowercase()
}

pub fn get_language(req: &HttpRequest) -> String {
    if let Some(accept_language) = req.headers().get("Accept-Language") {
        let accept_language: &str = accept_language.to_str().unwrap_or("en");

        let parts: Vec<String> = accept_language.split(';').map(|e| e.to_string()).collect();
        let lang_parts: Vec<String> = parts[0].split(',').map(|e| e.to_string()).collect();

        if lang_parts.len() == 2 {
            return lang_parts[1].clone();
        }
    }

    String::from("en")
}

pub fn get_language_texts(req: &HttpRequest) -> (String, Language) {
    let page_lang: String = get_language(req);
    (
        page_lang.clone(),
        get_langague_or(&page_lang, "en").expect("Cannot parse language."),
    )
}

pub fn get_scss_content(path: &str) -> grass::Result<String> {
    grass::from_path(
        path,
        &grass::Options::default().style(grass::OutputStyle::Compressed),
    )
}
