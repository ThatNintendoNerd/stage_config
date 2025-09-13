use libc2::app::GlobalStageParameter;
use serde::Deserialize;

use crate::config::Config;

/// The parameters for stage hazards.
#[derive(Deserialize)]
pub struct GimmickParam {
    /// Determines if stage hazards should be enabled.
    #[serde(default)]
    is_gimmick: bool,
}

/// Updates the parameters for stage hazards if the given stage identifier is assigned specialized parameters.
pub fn try_set_gimmick_param(stage_parameter: &mut GlobalStageParameter) {
    let stage_id = stage_parameter.stage_id();

    if let Some(param) = Config::get().gimmick_param.get(&stage_id) {
        stage_parameter.is_gimmick = param.is_gimmick;
    }
}
