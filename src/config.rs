use std::collections::{HashMap, HashSet};
use std::fs;

use hash40::Hash40;
use once_cell::sync::Lazy;
use serde::Deserialize;
use smash_stage::app::StageID;
use walkdir::WalkDir;

use crate::hooks::gravity::GravityParam;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = Config::new();

    for entry in WalkDir::new("sd:/ultimate/mods/")
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let mut entry_path = entry.into_path();

        if !arcropolis_api::is_mod_enabled(arcropolis_api::hash40(
            entry_path.to_str().unwrap_or_default(),
        )) {
            continue;
        }

        entry_path.push("config_stage.toml");

        if !entry_path.is_file() {
            continue;
        }

        if let Ok(string) = fs::read_to_string(&entry_path) {
            match toml::from_str(&string) {
                Ok(cfg) => config.merge(cfg),
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to parse TOML data from file '{}': {}",
                        module_path!(),
                        entry_path.display(),
                        error,
                    );
                }
            }
        }
    }

    config
});

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub new_dynamic_collisions: HashMap<StageID, HashSet<Hash40>>,

    #[serde(default)]
    pub is_flat_stage: HashMap<StageID, bool>,

    #[serde(default)]
    pub gravity_param: HashMap<StageID, GravityParam>,

    #[serde(default)]
    pub stage_additional_settings: HashMap<StageID, i8>,

    #[serde(default)]
    pub discard_stage_code: Vec<StageID>,
}

impl Config {
    fn new() -> Self {
        Self {
            new_dynamic_collisions: HashMap::new(),
            is_flat_stage: HashMap::new(),
            gravity_param: HashMap::new(),
            stage_additional_settings: HashMap::new(),
            discard_stage_code: Vec::new(),
        }
    }

    fn merge(&mut self, other: Self) {
        let Self {
            new_dynamic_collisions,
            is_flat_stage,
            gravity_param,
            stage_additional_settings,
            discard_stage_code,
        } = other;

        self.new_dynamic_collisions.extend(new_dynamic_collisions);
        self.is_flat_stage.extend(is_flat_stage);
        self.gravity_param.extend(gravity_param);
        self.stage_additional_settings
            .extend(stage_additional_settings);
        self.discard_stage_code.extend(discard_stage_code);
    }
}
