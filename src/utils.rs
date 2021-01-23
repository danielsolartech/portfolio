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
