use std::fs;

use chrono::Utc;
use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "index.stpl")]
struct IndexPage {
    title: &'static str,
    build_time: String,
}

pub fn build_index() -> std::io::Result<()> {
    let ctx = IndexPage {
        title: "Luke Carr",
        build_time: Utc::now().to_rfc3339(),
    };
    fs::write("out/index.html", ctx.render_once().unwrap())
}
