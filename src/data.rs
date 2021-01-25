use crate::utils::get_current_directory;
use serde::Deserialize;
use std::{collections::HashMap, fs};

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
pub struct Project {
    pub images: Vec<String>,
    pub category: usize,
    pub name: Languages,
    pub date: ProjectDate,
    pub description: Languages,
    pub links: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Projects {
    pub categories: Vec<Languages>,
    pub projects: Vec<Project>,
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
            Err(_) => Err(format!("Cannot parse file: {}", file_rute)),
        },
        Err(_) => Err(format!("Cannot read file: {}", file_rute)),
    }
}
