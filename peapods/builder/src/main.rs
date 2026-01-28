// `unwrap` more, why don't you
use std::path::PathBuf;
use handlebars::{Context, DirectorySourceOptionsBuilder, Handlebars};
use std::fs::File;
use std::process::Command;

const BUILD_DIR: &str = "../gh-pages";

#[derive(serde::Deserialize, serde::Serialize)]
struct PeapodMeta {
    title: String,
    pub_date: String,
    link: String,
    download: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PeapodData {
    peapods: Vec<PeapodMeta>,
}

fn setup_build_dir() {
    let out = Command::new("git")
        .args(["worktree", "add", BUILD_DIR, "main"])
        .output()
        .unwrap();
    // `worktree add` fails with 128 when it already exists.
    // ...we don't care.
    if ![0, 128].contains(&out.status.code().unwrap()) {
        eprintln!("{}", String::from_utf8(out.stderr).unwrap());
        panic!();
    }
}

fn main() {
    setup_build_dir();

    let mut handlebars = Handlebars::new();
    let dir_opts = DirectorySourceOptionsBuilder::default()
        .tpl_extension(".hbs")
        .build()
        .unwrap();
    handlebars.register_templates_directory("../templates", dir_opts).unwrap();

    let data_file = File::open("../peapods.yaml").unwrap();
    let mut peapods: PeapodData = serde_yaml::from_reader(data_file).unwrap();

    let render_template = |name, context| {
        let path: PathBuf = [BUILD_DIR, "peapods", name].iter().collect();
        let file = File::create(path).unwrap();
        handlebars.render_with_context_to_write(name, &context, file).unwrap();
    };
    render_template("index.html", Context::wraps(&peapods).unwrap());

    peapods.peapods.reverse(); // I don't know why I keep using Handlebars
    render_template("feed.rss", Context::wraps(&peapods).unwrap());

    println!("\nRendered to `../gh-pages`! Don't forget to:\n- Add the .ipuz file for download\n- Commit and push!");
}
