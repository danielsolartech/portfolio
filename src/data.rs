use crate::utils::get_current_directory;
use regex::Regex;
use serde::Deserialize;
use std::{path::Path, fs};

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

impl ProjectLink {
    pub fn format(&self, page_lang: &String) -> (String, String) {
        match self {
            ProjectLink::Single { name, url } => (name.clone(), url.clone()),
            ProjectLink::Multiple { name, url } => (
                match page_lang.as_str() {
                    "es" => name.es.clone(),
                    _ => name.en.clone(),
                },
                url.clone(),
            ),
        }
    }
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

    pub fn get_url(&self, lang: &String) -> String {
        let name = self.get_name(lang);

        name.replace("/", "")
            .replace(".", "")
            .replace(" ", "-")
            .replace("ñ", "n")
            .to_lowercase()
            .trim()
            .to_string()
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

    pub fn get_project(&self, name: &String, lang: &String) -> Option<Project> {
        for project in self.projects.iter() {
            if &project.get_url(lang) == name {
                return Some(project.clone());
            }
        }

        None
    }
}

impl Projects {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            projects: Vec::new(),
        }
    }

    pub fn get(&mut self) -> Result<(), String> {
        let file_rute: String = format!("{}assets/projects.json", get_current_directory());

        match fs::read_to_string(&file_rute) {
            Ok(projects_json) => match serde_json::from_str::<Projects>(&projects_json) {
                Ok(projects) => {
                    self.categories = projects.categories;
                    self.projects = projects.projects;
                    Ok(())
                },
                Err(e) => Err(format!("Cannot parse file: {} - {}", file_rute, e)),
            },
            Err(_) => Err(format!("Cannot read file: {}", file_rute)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlogPostDate {
    pub day: i64,
    pub month: usize,
    pub year: i64,
}

impl BlogPostDate {
    pub fn new() -> Self {
        Self {
            day: 0,
            month: 1,
            year: 2020,
        }
    }

    pub fn from(text: String) -> Self {
        let mut date = Self::new();

        let parts: Vec<String> = text.split("-").map(|e| e.to_string()).collect();
        if parts.len() == 3 {
            date.year = parts[0].parse().expect("Cannot parse year.");
            date.month = parts[1].parse().expect("Cannot parse month.");
            date.day = parts[2].parse().expect("Cannot parse day.");
        }

        date
    }

    pub fn to_string(&self) -> String {
        let month: String = format!("0{}", self.month);
        let day: String = format!("0{}", self.day);

        format!("{}-{}-{}", self.year, &month[month.len() - 2..], &day[day.len() - 2..])
    }
}

#[derive(Clone, Debug)]
pub struct BlogPost {
    pub url: String,
    pub lang: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub posted_date: BlogPostDate,

    pub titles: Vec<(usize, String)>,

    pub content: String,
    pub content_html: String,

    pub reading_time: i64,
}

impl BlogPost {
    pub fn get_full_url(&self) -> String {
        format!("{}/{}", self.posted_date.to_string(), self.url)
    }
}

#[derive(Clone, Debug)]
pub struct Blog {
    pub posts: Vec<BlogPost>,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            posts: Vec::new(),
        }
    }

    fn read_file(&mut self, file_rute: String) -> Result<(), String> {
        let file_path = Path::new(&file_rute);
        if !file_path.exists() || !file_path.is_file() {
            return Err(format!("{} does not exist or is not a file.", file_rute));
        }

        if !file_rute.ends_with(".md") {
            return Err(format!("{} is not a markdown file.", file_rute));
        }

        let url: String = file_path.file_name().expect("Cannot parse the file name.").to_str().unwrap().replace(".md", "");

        let parent_directory = file_path.parent().expect("Cannot get the parent directory.");
        let parent_name: &str = parent_directory.file_name().expect("Cannot parse the parent directory name.").to_str().unwrap_or("");

        let posted_date = BlogPostDate::from(parent_name.to_string());

        let blog_post_content: String = fs::read_to_string(&file_rute).expect("Cannot read file.");

        let table_expression = Regex::new(r"^(---(?P<text>[\s\S]+)---)").unwrap();
        let table_captures = table_expression.captures(&blog_post_content).unwrap();

        let blog_post_table: &str = &table_captures["text"].trim();
        let table_parts: Vec<&str> = blog_post_table.split("\n").collect();

        let mut lang: String = String::new();
        let mut title: String = String::new();
        let mut description: String = String::new();
        let mut image: String = String::new();

        for table_part in table_parts.iter() {
            let sub_parts: Vec<&str> = table_part.split(":").collect();
            if sub_parts.len() >= 2 {
                let value: String = sub_parts[1..].join(":").trim().to_string();

                match sub_parts[0].trim() {
                    "lang" => { lang = value; },
                    "title" => { title = value; },
                    "description" => { description = value; },
                    "image" => { image = value; },
                    _ => {},
                }
            }
        }

        let mut titles: Vec<(usize, String)> = Vec::new();

        let blog_post_content: String = blog_post_content.replace(&format!("---\n{}\n---", blog_post_table), "");
        let content: String = blog_post_content.trim().to_string();

        let title_expression = Regex::new(r"(?P<size>#{1,6})\s+(?P<text>[\w\s?¿¡!]+\n)").unwrap();
        for title_capture in title_expression.captures_iter(&content)  {
            let size: usize = title_capture["size"].trim().to_string().len();
            let text: String = title_capture["text"].split("\n").collect::<Vec<&str>>()[0].trim().to_string();

            titles.push((size, text));
        }

        let mut comrak_options = comrak::ComrakOptions::default();
        comrak_options.extension.header_ids = Some(String::new());

        let content_html = comrak::markdown_to_html(&content, &comrak_options);

        self.posts.push(BlogPost {
            url,
            lang,
            title,
            description,
            image,
            posted_date,
            titles,
            content: content.clone(),
            content_html,
            reading_time: (content.len() / 200).to_string().parse::<f32>().unwrap().ceil().to_string().parse().unwrap(),
        });

        Ok(())
    }

    fn read(&mut self, rute: String) -> Result<(), String> {
        let rute_path = Path::new(&rute);
        if !rute_path.exists() {
            Err(format!("{} does not exist.", rute))
        } else if rute_path.is_dir() {
            for entry in fs::read_dir(&rute_path).expect("Cannot read the directory.") {
                let entry = entry.expect("Cannot parse entry file.");
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    self.read(entry_path.display().to_string()).expect("Cannot read directory.");
                } else if entry_path.is_file() {
                    self.read_file(entry_path.display().to_string()).expect("Cannot read file.");
                }
            }

            Ok(())
        } else {
            self.read_file(rute)
        }
    }

    pub fn get(&mut self) -> Result<(), String> {
        self.read(format!("{}assets/blog", get_current_directory()))
    }

    pub fn get_post(&self, url: &String) -> Option<BlogPost> {
        for blog_post in self.posts.iter() {
            if &blog_post.get_full_url() == url {
                return Some(blog_post.clone());
            }
        }

        None
    }
}
