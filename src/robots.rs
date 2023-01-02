use std::fs;

pub fn build_robots() -> std::io::Result<()> {
    fs::write("out/robots.txt", concat!("User-agent: *", "Allow: /"))
}
