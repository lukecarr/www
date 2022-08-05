use chrono::offset::Utc;
use std::fs;
use std::time::Instant;
use chrono::TimeZone;
use sailfish::TemplateOnce;

fn main() {
    let now = Instant::now();

    fs::create_dir_all("out").expect("Failed to create ouput directory");

    let index = IndexPage {
        age: get_age(),
        title: "Luke Carr".to_owned()
    };
    build_index(index).expect("Failed to build index page");

    let links = LinksPage {
        title: "Links :: Luke Carr".to_owned()
    };
    build_links(links).expect("Failed to build links page");

    println!("Compiled site successfully in {:.1}ms!", (now.elapsed().as_nanos() as f64) / 1e6);
}

fn get_age() -> u8 {
    let utc = Utc;
    let dob = utc.ymd(2001, 7, 30);
    let age = Utc::today().signed_duration_since(dob).num_days() as f64 / 365.25;
    return age as u8;
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct IndexPage {
    age: u8,
    title: String
}

fn build_index(ctx: IndexPage) -> std::io::Result<()> {
    fs::write("out/index.html", ctx.render_once().unwrap())
}

#[derive(TemplateOnce)]
#[template(path = "links.stpl")]
struct LinksPage {
    title: String
}

fn build_links(ctx: LinksPage) -> std::io::Result<()> {
    fs::write("out/links.html", ctx.render_once().unwrap())
}
