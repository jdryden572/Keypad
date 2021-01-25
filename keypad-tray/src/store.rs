use std::{
    fs::{create_dir_all, File},
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use app_dirs::{get_app_dir, AppDataType, AppDirsError, AppInfo};
use serde_json::{from_reader, to_writer_pretty};
use thiserror::Error;

use crate::models::Profile;

const APP_INFO: AppInfo = AppInfo {
    name: "KeypadControl",
    author: "JDryden",
};
const PROFILES_DIR: &str = "profiles";
const PROFILES_FILE: &str = "profiles.json";

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Error getting app directory: {0}")]
    AppDirsError(#[from] AppDirsError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde JSON error: {0}")]
    SerdeError(#[from] serde_json::error::Error),
}

pub fn load_profiles() -> Result<Vec<Profile>, StoreError> {
    let path = get_or_create_profiles_file()?;
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let profiles = match from_reader(reader) {
        Ok(profiles) => profiles,
        Err(_) => {
            let profiles = Vec::new();
            store_profiles(&profiles)?;
            profiles
        }
    };
    Ok(profiles)
}

pub fn store_profiles(profiles: &Vec<Profile>) -> Result<(), StoreError> {
    let path = get_or_create_profiles_file()?;
    let mut file = File::create(&path)?;
    to_writer_pretty(&file, &profiles)?;
    file.flush()?;
    Ok(())
}

fn get_or_create_profiles_file() -> Result<PathBuf, StoreError> {
    let app_dir = get_app_dir(AppDataType::UserConfig, &APP_INFO, PROFILES_DIR)?;
    create_dir_all(&app_dir)?;
    let path = app_dir.join(&PROFILES_FILE);
    if !path.exists() {
        let _ = File::create(&path)?;
    }
    Ok(path)
}
