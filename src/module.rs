use crate::components;
use crate::utils::*;

fn get_base_path(module: &str, project: &Option<String>) -> String {
    match project {
        Some(project_name) => format!("projects/{}/modules/{}", project_name, module),
        None => format!("modules/{}", module),
    }
}

pub fn new(module: &str, project: &Option<String>) {
    let base_path = get_base_path(module, project);
    // Create module directory
    make_directory(&base_path);

    // Populate module with default files
    make_file(
        &format!("{}/components.toml", base_path),
        "module/components.toml",
        &liquid::object!({ "module_name": module }),
    );
    make_file(
        &format!("{}/main.cpp", base_path),
        "module/main.cpp",
        &liquid::object!({}),
    );
    make_file(
        &format!("{}/CMakeLists.txt", base_path),
        "module/CMakeLists.txt",
        &liquid::object!({}),
    );
}

pub fn build(module: &str, project: &Option<String>) {
    let base_path = get_base_path(module, project);
    // components::generate(&base_path, true, true);
    println!(
        "NOT IMPLEMENTED: module {} build (project={})",
        module,
        project.as_deref().unwrap_or("None")
    );
}
