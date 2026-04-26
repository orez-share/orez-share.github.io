// `unwrap` more, why don't you
use std::path::PathBuf;
use handlebars::{Context, DirectorySourceOptionsBuilder, Handlebars, handlebars_helper};
use std::fs::File;
use std::process::Command;

// TODO: really we want `${REPO_ROOT}/gh-pages`.
// getting this sounds like a pain.
const BUILD_DIR: &str = "../../gh-pages";

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="lowercase")]
enum CrosswordVariant {
    Peapod,
    Looping,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct CrosswordMeta {
    title: String,
    pub_date: String,
    link: Option<String>,
    mirror: Option<String>,
    download: String,
    variants: Vec<CrosswordVariant>,

    // XXX: if we're doing crap like this we should really have separate ser and deser structs,
    // but we can rebuild that bridge once we exceed the weight limit.
    #[serde(skip_deserializing)]
    play_link: Option<String>,
}

impl CrosswordMeta {
    fn populate_link(&mut self) {
        if self.link.is_some() { return }
        // XXX: probably we shouldn't be guessing like this.
        // Who cares.
        let Some((name, extension)) = self.download.split_once('.') else { return };
        let can_link = extension == "ipuz" &&
            name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'));
        if !can_link { return }
        self.link = Some(format!("/crosswords/play/#/{}", name));
    }

    fn post_deser(&mut self) {
        self.populate_link();
        let link = self.link.as_ref().map(|link| absolute_url(link));
        self.play_link = link.or(self.mirror.clone());
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct CrosswordData {
    crosswords: Vec<CrosswordMeta>,
}

fn absolute_url(link: &str) -> String {
    if link.starts_with('/') {
        return format!("https://orez-share.github.io{link}");
    }
    link.to_owned()
}

fn setup_build_dir() {
    let out = Command::new("git")
        .args(["worktree", "add", BUILD_DIR, "gh-pages"])
        .output()
        .unwrap();
    // `worktree add` fails with 128 when it already exists.
    // ...we don't care.
    if ![0, 128].contains(&out.status.code().unwrap()) {
        eprintln!("{}", String::from_utf8(out.stderr).unwrap());
        panic!();
    }
}

fn render_template(handlebars: &Handlebars, name: &str, context: Context) {
    let path: PathBuf = [BUILD_DIR, name].iter().collect();
    let file = File::create(path).unwrap();
    handlebars.render_with_context_to_write(name, &context, file).unwrap();
}

fn render_peapods(handlebars: &Handlebars, crosswords: &CrosswordData) {
    let mut peapods = crosswords.clone();
    peapods.crosswords.retain(|x| x.variants.contains(&CrosswordVariant::Peapod));

    render_template(handlebars, "peapods/index.html", Context::wraps(&peapods).unwrap());

    peapods.crosswords.reverse(); // I don't know why I keep using Handlebars
    render_template(handlebars, "peapods/feed.rss", Context::wraps(&peapods).unwrap());
}

fn render_looping(handlebars: &Handlebars, crosswords: &CrosswordData) {
    let mut looping = crosswords.clone();
    looping.crosswords.retain(|x| x.variants.contains(&CrosswordVariant::Looping));

    render_template(handlebars, "crosswords/looping/index.html", Context::wraps(&looping).unwrap());

    // No looping crossword rss feed.
    // The peapod one is grandfathered in, for now.
}

fn render_crosswords(handlebars: &Handlebars, crosswords: &CrosswordData) {
    let mut looping = crosswords.clone();

    render_template(handlebars, "crosswords/index.html", Context::wraps(&looping).unwrap());

    looping.crosswords.reverse();
    render_template(handlebars, "crosswords/feed.rss", Context::wraps(&looping).unwrap());
}

fn main() {
    setup_build_dir();

    let mut handlebars = Handlebars::new();
    let dir_opts = DirectorySourceOptionsBuilder::default()
        .tpl_extension(".hbs")
        .build()
        .unwrap();
    handlebars.register_templates_directory("../templates", dir_opts).unwrap();
    handlebars_helper!(eq: |x: str, y: str| x == y);

    let data_file = File::open("../crosswords.yaml").unwrap();
    let mut crosswords: CrosswordData = serde_yaml::from_reader(data_file).unwrap();
    crosswords.crosswords.iter_mut().for_each(|x| x.post_deser());

    render_peapods(&handlebars, &crosswords);
    render_looping(&handlebars, &crosswords);
    render_crosswords(&handlebars, &crosswords);

    println!("\nRendered to `/gh-pages`! Don't forget to:\n- Add the .ipuz file for download\n- Commit and push!");
}
