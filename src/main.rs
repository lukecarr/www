use std::fs;

mod index;
use index::build_index;
mod links;
use links::{build_links, Link};
mod robots;
use robots::build_robots;

fn main() {
    fs::create_dir_all("out").expect("Failed to create ouput directory");

    build_index().expect("Failed to build index page");
    build_robots().expect("Failed to build robots.txt");

    let links = vec![
        Link {
            href: "https://sr.ht/~carr",
            text: "Sourcehut (~carr)",
        },
        Link {
            href: "https://github.com/lukecarr",
            text: "GitHub (lukecarr)",
        },
        Link {
            href: "https://gitlab.com/lukecarr",
            text: "GitLab (lukecarr)",
        },
        Link {
            href: "https://linkedin.com/in/luke-j-carr",
            text: "LinkedIn (luke-j-carr)",
        },
    ];
    build_links(links).expect("Failed to build links page");
}
