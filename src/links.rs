use std::fs;

use chrono::Utc;
use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "links.stpl")]
struct LinksPage {
    title: &'static str,
    build_time: String,
    links: Vec<Link>,
}

pub struct Link {
    pub href: &'static str,
    pub text: &'static str,
}

pub fn build_links(links: Vec<Link>) -> std::io::Result<()> {
    let ctx = LinksPage {
        title: "Links :: Luke Carr",
        build_time: Utc::now().to_rfc3339(),
        links,
    };
    fs::write("out/links.html", ctx.render_once().unwrap())
}
