use rust_embed::RustEmbed;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

fn template(filename: &str) -> liquid::Template {
    let file = Templates::get(&filename).unwrap();
    let content = std::str::from_utf8(file.data.as_ref());
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(content.unwrap())
        .unwrap();
    template
}

pub fn template_to_str(template_file: &str, template_data: &liquid::Object) -> String {
    template(template_file).render(template_data).unwrap()
}

pub fn make_file(output_file: &str, template_file: &str, template_data: &liquid::Object) {
    let config = template_to_str(template_file, template_data);
    let mut file = File::create(output_file).unwrap();
    file.write_all(config.as_bytes()).unwrap();
}

pub fn make_file_with_str(output_file: &str, contents: &str) {
    let mut file = File::create(output_file).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn make_directory(directory_path: &str) {
    DirBuilder::new()
        .recursive(true)
        .create(directory_path)
        .unwrap();
}
