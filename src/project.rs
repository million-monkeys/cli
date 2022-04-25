use crate::events;
use crate::utils::*;
use clap::ArgEnum;
use colored::*;
use std::process::Command;
use std::{env, fs, io};
use strum_macros::Display;

#[derive(Display, Copy, Clone, ArgEnum)]
pub enum RunBuild {
    Release,
    Debug,
}

#[derive(Display, Copy, Clone, ArgEnum)]
pub enum LogLevel {
    Off,
    Crticial,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub fn create(project: &str) {
    let project_dir = get_project_dir(project);

    println!("Creating: {}", project_dir.blue());
    // Create directory structure
    for dir in ["scenes", "props", "features"] {
        // println!("Creating: projects/{}/{}", name.blue(), dir.blue());
        make_directory(&format!("{}/{}", project_dir, dir));
    }
    make_file(
        &format!("{}/.gitignore", project_dir),
        "project/gitignore",
        &liquid::object!({}),
    );
    // Game configuration
    make_file(
        &format!("{}/config.toml", project_dir),
        "project/config.toml",
        &liquid::object!({ "project_name": project }),
    );
    make_file(
        &format!("{}/game.toml", project_dir),
        "project/game.toml",
        &liquid::object!({}),
    );
    make_file(
        &format!("{}/events.toml", project_dir),
        "project/events.toml",
        &liquid::object!({}),
    );
    // Default scene
    make_file(
        &format!("{}/scenes/default.toml", project_dir),
        "project/default_scene.toml",
        &liquid::object!({}),
    );
    // Player prop
    make_directory(&format!("{}/props/player/assets", project_dir));
    make_directory(&format!("{}/props/player/scripts", project_dir));
    make_file(
        &format!("{}/props/player/entity.toml", project_dir),
        "project/player_template.toml",
        &liquid::object!({}),
    );
    make_file(
        &format!("{}/props/player/scripts/player.lua", project_dir),
        "project/player_script.lua",
        &liquid::object!({}),
    );
    // Game setup "feature"
    make_directory(&format!("{}/features/game/assets", project_dir));
    make_directory(&format!("{}/features/game/scripts", project_dir));
    make_file(
        &format!("{}/features/game/config.toml", project_dir),
        "project/feature_config.toml",
        &liquid::object!({}),
    );
    make_file(
        &format!("{}/features/game/scripts/run.lua", project_dir),
        "project/feature_script.lua",
        &liquid::object!({}),
    );
}

fn _is_root_dir() -> io::Result<bool> {
    let mut path = env::current_dir()?;
    path.push("engine");
    let metadata = fs::metadata(path)?;
    Ok(metadata.is_dir())
}
fn is_root_dir() -> bool {
    _is_root_dir().unwrap_or(false)
}

fn get_project_dir(project: &str) -> String {
    if is_root_dir() {
        format!("projects/{}", project)
    } else {
        format!("../../projects/{}", project)
    }
}

pub fn build(project: &str, builddir: &Option<String>) {
    let project_dir = get_project_dir(project);
    let output_dir: String = match builddir {
        Some(dir) => dir.to_string(),
        None => format!("{}/build", project_dir),
    };
    make_directory(&format!("{}", output_dir));
    events::generate(
        format!("{}/events.toml", project_dir).as_str(),
        true,
        false,
        &output_dir,
    );
    println!("WIP: project {} build", project);
}

pub fn dev(project: &str) {}

fn _run(root_dir: &str, project_dir: &str, binary: &str, log_level: &LogLevel) -> io::Result<bool> {
    println!("Running {}", binary.blue());
    let output = Command::new(binary)
        .args(["-g", &format!("{}/engine/src/lua/engine", root_dir)])
        .args(["-g", project_dir])
        .args(["--init", &format!("{}/config.toml", project_dir)])
        .args([
            "-l",
            match log_level {
                LogLevel::Trace => "trace",
                LogLevel::Debug => "debug",
                LogLevel::Info => "info",
                LogLevel::Warn => "warn",
                LogLevel::Error => "error",
                LogLevel::Crticial => "crticial",
                LogLevel::Off => "off",
            },
        ])
        .spawn()?;
    Ok(true)
}

pub fn run(project: &str, engine_build: &RunBuild, log_level: &Option<LogLevel>) {
    let binary = match engine_build {
        RunBuild::Debug => "game-debug",
        RunBuild::Release => "game-release",
    };
    let log_level = log_level.unwrap_or(match engine_build {
        RunBuild::Debug => LogLevel::Debug,
        RunBuild::Release => LogLevel::Info,
    });
    if is_root_dir() {
        _run(
            ".",
            &format!("projects/{}", project),
            &format!("engine/{}", binary),
            &log_level,
        )
        .expect("Failed to execute");
    } else {
        _run(
            "../..",
            &format!("../../projects/{}/", project),
            &format!("../../engine/{}", binary),
            &log_level,
        )
        .expect("Failed to execute");
    };
}

pub fn release(project: &str) {
    println!("NOT IMPLEMENTED: project {} release", project);
}
