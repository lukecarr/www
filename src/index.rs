use std::fs;

use chrono::{TimeZone, Utc};
use sailfish::TemplateSimple;

fn get_age() -> u8 {
    let utc = Utc;
    let dob = utc
        .with_ymd_and_hms(2001, 7, 30, 0, 0, 0)
        .single()
        .expect("Failed to parse birthday");
    let age = Utc::now().signed_duration_since(dob).num_days() as f64 / 365.25;
    age as u8
}

#[derive(TemplateSimple)]
#[template(path = "index.stpl")]
struct IndexPage {
    age: u8,
    title: &'static str,
    build_time: String,
}

pub fn build_index() -> std::io::Result<()> {
    let ctx = IndexPage {
        title: "Luke Carr",
        build_time: Utc::now().to_rfc3339(),
        age: get_age(),
    };
    fs::write("out/index.html", ctx.render_once().unwrap())
}
