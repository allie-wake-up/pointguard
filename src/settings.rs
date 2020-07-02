use config::{Config, ConfigError, Environment, File};
use directories::{ProjectDirs, UserDirs};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Settings {
    pub dir: PathBuf,
    pub clip_time: u32,
    pub generated_length: u32,
    pub editor: String,
}

const CLIP_TIME: u32 = 45;
const GENERATED_LENGTH: u32 = 25;
const EDITOR: &str = "vim";

fn get_point_guard_default_dir(proj_dirs: &Option<ProjectDirs>) -> Option<PathBuf> {
    match proj_dirs {
        Some(proj_dirs) => {
            let path = PathBuf::from(proj_dirs.data_dir());
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_pass_default_dir() -> Option<PathBuf> {
    match UserDirs::new() {
        Some(user_dirs) => {
            let mut path = PathBuf::from(user_dirs.home_dir());
            path.push(".password-store");
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        }
        None => None,
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        let proj_dirs = ProjectDirs::from("dev", "Point Guard", "pg");
        if let Some(proj_dirs) = &proj_dirs {
            let mut path = PathBuf::from(proj_dirs.config_dir());
            path.push("config.toml");
            settings.merge(File::from(path).required(false))?;
        }
        settings
            .merge(Environment::with_prefix("PASSWORD_STORE"))?
            .merge(Environment::with_prefix("POINT_GUARD"))?;

        let mut map = settings.try_into::<HashMap<String, String>>()?;

        let dir = match map.remove("dir") {
            Some(dir) => Some(PathBuf::from(dir)),
            None => None,
        };
        let dir = dir.or_else(|| get_point_guard_default_dir(&proj_dirs));
        let dir = dir.or_else(get_pass_default_dir);
        if dir.is_none() {
            return Err(ConfigError::Message(String::from(
                "Password directory could not be found",
            )));
        }

        Ok(Settings {
            dir: dir.unwrap(), // can unwrap because we return Err() above if None
            clip_time: match map.get("clip_time") {
                Some(clip_time) => clip_time.parse().unwrap_or(CLIP_TIME),
                None => CLIP_TIME,
            },
            generated_length: match map.get("generated_length") {
                Some(length) => length.parse().unwrap_or(GENERATED_LENGTH),
                None => GENERATED_LENGTH,
            },
            editor: map.remove("editor").unwrap_or_else(|| String::from(EDITOR)),
        })
    }
}
