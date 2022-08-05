use chrono::offset::Utc;
use std::fs;
use std::time::Instant;
use chrono::TimeZone;
use sailfish::TemplateOnce;

fn main() {
    let now = Instant::now();
    let build_time = Utc::now();

    fs::create_dir_all("out").expect("Failed to create ouput directory");

    let index = IndexPage {
        age: get_age(),
        title: "Luke Carr".to_owned(),
        build_time: build_time.to_rfc3339(),
    };
    build_index(index).expect("Failed to build index page");

    let links = LinksPage {
        title: "Links :: Luke Carr".to_owned(),
        build_time: build_time.to_rfc3339(),
    };
    build_links(links).expect("Failed to build links page");

    build_robots().expect("Failed to build robots.txt");

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
    title: String,
    build_time: String,
}

fn build_index(ctx: IndexPage) -> std::io::Result<()> {
    fs::write("out/index.html", ctx.render_once().unwrap())
}

#[derive(TemplateOnce)]
#[template(path = "links.stpl")]
struct LinksPage {
    title: String,
    build_time: String,
}

fn build_links(ctx: LinksPage) -> std::io::Result<()> {
    fs::write("out/links.html", ctx.render_once().unwrap())
}

fn build_robots() -> std::io::Result<()> {
    fs::write("out/robots.txt", concat!("User-agent: *", "Allow: /"))
}
