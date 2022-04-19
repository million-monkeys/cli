use crate::events;
use crate::utils::*;
use clap::ArgEnum;
use colored::*;
use strum_macros::Display;

#[derive(Display, Copy, Clone, ArgEnum)]
pub enum RunBuild {
    Release,
    Debug,
}

pub fn create(project: &str) {
    println!("Creating: projects/{}", project.blue());
    // Create directory structure
    for dir in ["scenes", "props", "features"] {
        // println!("Creating: projects/{}/{}", name.blue(), dir.blue());
        make_directory(&format!("projects/{}/{}", project, dir));
    }
    make_file(
        &format!("projects/{}/.gitignore", project),
        "project/gitignore",
        &liquid::object!({}),
    );
    // Game configuration
    make_file(
        &format!("projects/{}/config.toml", project),
        "project/config.toml",
        &liquid::object!({ "project_name": project }),
    );
    make_file(
        &format!("projects/{}/game.toml", project),
        "project/game.toml",
        &liquid::object!({}),
    );
    make_file(
        &format!("projects/{}/events.toml", project),
        "project/events.toml",
        &liquid::object!({}),
    );
    // Default scene
    make_file(
        &format!("projects/{}/scenes/default.toml", project),
        "project/default_scene.toml",
        &liquid::object!({}),
    );
    // Player prop
    make_directory(&format!("projects/{}/props/player/assets", project));
    make_directory(&format!("projects/{}/props/player/scripts", project));
    make_file(
        &format!("projects/{}/props/player/entity.toml", project),
        "project/player_template.toml",
        &liquid::object!({}),
    );
    make_file(
        &format!("projects/{}/props/player/scripts/player.lua", project),
        "project/player_script.lua",
        &liquid::object!({}),
    );
    // Game setup "feature"
    make_directory(&format!("projects/{}/features/game/assets", project));
    make_directory(&format!("projects/{}/features/game/scripts", project));
    make_file(
        &format!("projects/{}/features/game/config.toml", project),
        "project/feature_config.toml",
        &liquid::object!({}),
    );
    make_file(
        &format!("projects/{}/features/game/scripts/run.lua", project),
        "project/feature_script.lua",
        &liquid::object!({}),
    );
}

pub fn build(project: &str, builddir: &Option<String>) {
    let output_dir: String = match builddir {
        Some(dir) => dir.to_string(),
        None => format!("projects/{}/build", project),
    };
    make_directory(&format!("{}", output_dir));
    events::generate(
        format!("projects/{}/events.toml", project).as_str(),
        true,
        false,
        &output_dir,
    );
    println!("WIP: project {} build", project);
}

pub fn dev(project: &str) {}

pub fn run(project: &str, engine_build: &RunBuild) {
    println!(
        "NOT IMPLEMENTED: project {} run (build={})",
        project,
        engine_build.to_string()
    );
}

pub fn release(project: &str) {
    println!("NOT IMPLEMENTED: project {} release", project);
}
