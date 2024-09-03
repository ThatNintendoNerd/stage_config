use std::collections::{HashMap, HashSet};
use std::fs;

use hash40::Hash40;
use once_cell::sync::Lazy;
use serde::Deserialize;
use smash_stage::app::StageID;
use walkdir::WalkDir;

use crate::service::{GimmickParam, GravityParam};

/// The container for deserializable plugin settings.
#[derive(Default, Deserialize)]
pub struct Config {
    /// The collection of stage identifiers assigned a set of model names to associate with dynamic collisions.
    #[serde(default)]
    pub new_dynamic_collisions: HashMap<StageID, HashSet<Hash40>>,

    /// The collection of stage identifiers assigned a Boolean flag determining if it should flatten battle objects.
    #[serde(default)]
    pub is_flat_stage: HashMap<StageID, bool>,

    /// The collection of stage identifiers assigned specialized gravity parameters.
    #[serde(default)]
    pub gravity_param: HashMap<StageID, GravityParam>,

    /// The collection of stage identifiers assigned a behavior-altering numeric setting from spirit battles.
    #[serde(default)]
    pub stage_additional_settings: HashMap<StageID, i8>,

    /// The collection of stage identifiers which should discard all specialized programming.
    #[serde(default)]
    pub discard_stage_code: Vec<StageID>,

    /// The collection of stage identifiers assigned specialized stage hazards parameters.
    #[serde(default)]
    pub gimmick_param: HashMap<StageID, GimmickParam>,
}

impl Config {
    /// Constructs a new instance of `Config`.
    fn new() -> Self {
        let mut config = Config::default();

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

            match fs::read_to_string(&entry_path) {
                Ok(string) => match toml::from_str(&string) {
                    Ok(cfg) => config.merge(cfg),
                    Err(error) => {
                        eprintln!(
                            "[{}] Failed to parse TOML file data from '{}': {}",
                            module_path!(),
                            entry_path.display(),
                            error,
                        );
                    }
                },
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to read TOML file data from '{}': {}",
                        module_path!(),
                        entry_path.display(),
                        error,
                    );
                }
            }
        }

        config
    }

    /// Returns a reference to a `Lazy` containing the current instance of `Config`.
    pub fn get() -> &'static Lazy<Self> {
        static INSTANCE: Lazy<Config> = Lazy::new(Config::new);

        &INSTANCE
    }

    /// Merges the contents of `self` with the contents of `other`.
    fn merge(&mut self, other: Self) {
        let Self {
            new_dynamic_collisions,
            is_flat_stage,
            gravity_param,
            stage_additional_settings,
            discard_stage_code,
            gimmick_param,
        } = other;

        self.new_dynamic_collisions.extend(new_dynamic_collisions);
        self.is_flat_stage.extend(is_flat_stage);
        self.gravity_param.extend(gravity_param);
        self.stage_additional_settings
            .extend(stage_additional_settings);
        self.discard_stage_code.extend(discard_stage_code);
        self.gimmick_param.extend(gimmick_param);
    }
}
