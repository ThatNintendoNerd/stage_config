use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    sync::LazyLock,
};

use hash40::Hash40;
use libc2::app::StageID;
use serde::Deserialize;
use walkdir::WalkDir;

/// The container for deserializable plugin settings.
#[derive(Default, Deserialize)]
pub struct Config {
    /// The collection of stage identifiers assigned a set of model names to associate with dynamic collisions.
    #[serde(default)]
    pub new_dynamic_collisions: HashMap<StageID, HashSet<Hash40>>,

    /// The collection of stage identifiers assigned a Boolean determining if it should flatten battle objects.
    #[serde(default)]
    pub is_flat_stage: HashMap<StageID, bool>,

    /// The collection of stage identifiers assigned specialized gravity parameters.
    #[serde(default)]
    pub gravity_param: HashMap<StageID, ConfigGravityParameter>,

    /// The collection of stage identifiers assigned a behavior-altering numeric setting from spirit battles.
    #[serde(default)]
    pub stage_additional_settings: HashMap<StageID, i8>,

    /// The collection of stage identifiers to discard all specialized programming for.
    #[serde(default)]
    pub discard_stage_code: Vec<StageID>,

    /// The collection of stage identifiers assigned specialized stage hazards parameters.
    #[serde(default)]
    pub gimmick_param: HashMap<StageID, ConfigGimmickParameter>,
}

impl Config {
    /// Creates a new `Config`.
    fn new() -> Self {
        let mut config = Config::default();

        for entry in WalkDir::new("sd:/ultimate/mods/")
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let mut path = entry.into_path();
            let path_hash = arcropolis_api::hash40(path.to_str().unwrap());

            if !arcropolis_api::is_mod_enabled(path_hash) {
                continue;
            }

            path.push("config_stage.toml");

            if !path.is_file() {
                continue;
            }

            config.read_mut(path);
        }

        config
    }

    /// Returns a reference to the lazily initialized `Config`.
    pub const fn get() -> &'static LazyLock<Self> {
        static INSTANCE: LazyLock<Config> = LazyLock::new(Config::new);

        &INSTANCE
    }

    /// Reads and deserializes a configuration file from disk, extending `self` with it.
    fn read_mut<P: AsRef<Path>>(&mut self, path: P) {
        match fs::read_to_string(&path) {
            Ok(string) => match toml::from_str(&string) {
                Ok(config) => self.extend(config),
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to deserialize configuration file from '{}': {error}",
                        module_path!(),
                        path.as_ref().display(),
                    );
                }
            },
            Err(error) => {
                eprintln!(
                    "[{}] Failed to read configuration file from '{}': {error}",
                    module_path!(),
                    path.as_ref().display(),
                );
            }
        }
    }

    /// Extends `self` with the contents of `other`.
    fn extend(&mut self, other: Self) {
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

/// The parameters for gravity.
#[derive(Deserialize)]
pub struct ConfigGravityParameter {
    /// Determines if the stage should assume a flat gravitational plane.
    #[serde(default)]
    pub is_gravity_normal: bool,

    /// The position of the gravitational force.
    #[serde(default)]
    pub pos: Option<ConfigGravityPosition>,
}

/// The position of the gravitational force.
#[derive(Deserialize)]
pub struct ConfigGravityPosition {
    /// The position along the x-axis.
    #[serde(default)]
    pub x: f32,

    /// The position along the y-axis.
    #[serde(default)]
    pub y: f32,
}

/// The parameters for stage hazards.
#[derive(Deserialize)]
pub struct ConfigGimmickParameter {
    /// Determines if stage hazards should be enabled.
    #[serde(default)]
    pub is_gimmick: bool,
}
