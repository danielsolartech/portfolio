mod error404;
pub use error404::Error404;
mod home;
pub use home::Home;
mod projects;
pub use projects::Projects;

use crate::utils::{get_current_directory, get_scss_content};
use actix_web::{dev::HttpResponseBuilder, http::StatusCode, HttpResponse};

pub fn render_scss(name: &str) -> HttpResponse {
    let scss_rute: String = format!("{}assets/scss/{}.scss", get_current_directory(), name);

    match get_scss_content(&scss_rute) {
        Ok(css_content) => HttpResponseBuilder::new(StatusCode::OK)
            .content_type("text/css")
            .body(css_content),
        Err(e) => {
            println!("Template SCSS ({}): {} ", name, e);
            HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                .body(format!("Cannot parse SCSS file: {}", scss_rute))
        }
    }
}
