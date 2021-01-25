use crate::languages::{Language, PageTexts};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home<'a> {
    pub page_id: &'a str,

    pub page_lang: String,
    pub page_texts: PageTexts,
    pub texts: Language,

    pub page_url: String,
    pub page_keywords: &'a str,
    pub page_image: String,

    pub heart_svg: &'a str,
}
