use crate::utils::get_current_directory;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Languages {
    pub en: String,
    pub es: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProjectDate {
    pub day: Option<i64>,
    pub month: usize,
    pub year: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ProjectLink {
    #[serde(rename = "single")]
    Single { name: String, url: String },
    #[serde(rename = "multiple")]
    Multiple { name: Languages, url: String },
}

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub images: Vec<String>,
    pub category: usize,
    pub name: Languages,
    pub date: ProjectDate,
    pub description: Languages,
    pub links: Option<Vec<ProjectLink>>,
}

impl Project {
    pub fn get_main_image(&self) -> String {
        if self.images.len() >= 1 {
            self.images[0].clone()
        } else {
            String::new()
        }
    }

    pub fn get_name(&self, lang: &String) -> String {
        if lang == "es" {
            self.name.es.clone()
        } else {
            self.name.en.clone()
        }
    }

    pub fn get_description(&self, lang: &String) -> String {
        if lang == "es" {
            self.description.es.clone()
        } else {
            self.description.en.clone()
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Projects {
    pub categories: Vec<Languages>,
    pub projects: Vec<Project>,
}

impl Projects {
    pub fn get_category(&self, category_index: usize, page_lang: &String) -> String {
        let page_category = &self.categories[category_index];

        if page_lang == "es" {
            page_category.es.clone()
        } else {
            page_category.en.clone()
        }
    }
}

impl Projects {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            projects: Vec::new(),
        }
    }
}

pub fn get_projects() -> Result<Projects, String> {
    let file_rute: String = format!("{}assets/projects.json", get_current_directory());

    match fs::read_to_string(&file_rute) {
        Ok(projects_json) => match serde_json::from_str(&projects_json) {
            Ok(projects) => Ok(projects),
            Err(e) => Err(format!("Cannot parse file: {} - {}", file_rute, e)),
        },
        Err(_) => Err(format!("Cannot read file: {}", file_rute)),
    }
}
