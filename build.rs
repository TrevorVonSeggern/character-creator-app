//use std::env::set_current_dir;
use std::path::PathBuf;

use walkdir::DirEntry;
use walkdir::WalkDir;

fn main() {
    // set_current_dir("src").expect("Unable to find the src directory.");
    WalkDir::new(".")
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|x| !is_hidden(x))
        .filter(|x| only_sass_files(x))
        .for_each(|x| sass_to_css(x.path().display().to_string()));
}

fn sass_to_css(path: String) {
    println!("cargo:rerun-if-changed={}", path);
    let pbuffpath = path.clone();
    let contents = std::fs::read_to_string(path)
        .expect("Unable to read sass file.");

    let css = grass_compiler::from_string(
        contents,
        &grass_compiler::Options::default().style(grass_compiler::OutputStyle::Compressed).load_path("./assets")
    ).expect("sass should compile.");

    let mut pbuff = PathBuf::from(pbuffpath);
    pbuff.set_extension("css");
    std::fs::write(pbuff.to_str().expect("unable to change extension"), css)
        .expect("Unable to write css file.");
}


fn only_sass_files(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with(".sass"))
         .unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
