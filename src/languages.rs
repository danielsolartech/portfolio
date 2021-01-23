use crate::utils::get_current_directory;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
pub struct PageTexts {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct HeaderTexts {
    pub nav_home: String,
    pub nav_projects: String,
    pub nav_blog: String,
    pub nav_contact: String,
}

#[derive(Debug, Deserialize)]
pub struct AboutTexts {
    pub profession: String,
    pub about_me_title: String,
    pub messages: Vec<String>,
    pub skills_title: String,
}

#[derive(Debug, Deserialize)]
pub struct Error404Texts {
    pub error_title: String,
    pub error_message: String,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub pages: HashMap<String, PageTexts>,
    pub header: HeaderTexts,
    pub about: AboutTexts,
    pub error404: Error404Texts,
    pub footer: String,
}

pub fn get_language(lang: &String) -> Result<Language, String> {
    let file_rute: String = format!(
        "{}templates/languages/{}.json",
        get_current_directory(),
        lang
    );
    match fs::read_to_string(&file_rute) {
        Ok(language_json) => match serde_json::from_str(&language_json) {
            Ok(language) => Ok(language),
            Err(_) => Err(format!("Cannot parse file: {}", file_rute)),
        },
        Err(_) => Err(format!("Cannot read file: {}", file_rute)),
    }
}

pub fn get_langague_or(lang: &String, or: &str) -> Result<Language, String> {
    match get_language(lang) {
        Ok(language) => Ok(language),
        Err(_) => get_language(&or.to_string()),
    }
}
