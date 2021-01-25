use crate::{data::Projects as DataProjects, languages::{Language, PageTexts}};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "projects.stpl")]
pub struct Projects<'a> {
    pub page_id: &'a str,

    pub page_lang: String,
    pub page_texts: PageTexts,
    pub texts: Language,

    pub page_url: String,
    pub page_keywords: &'a str,
    pub page_image: String,

    pub projects: DataProjects,

    pub heart_svg: &'a str,
}
