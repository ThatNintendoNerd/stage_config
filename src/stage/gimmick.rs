use libc2::app::GlobalStageParameter;

use crate::config::Config;

/// Updates stage hazard settings based on parameters assigned to the working stage identifier.
pub fn set_gimmick_param(stage_parameter: &mut GlobalStageParameter) {
    let stage_id = stage_parameter.stage_id();
    let Some(param) = Config::get().gimmick_param.get(stage_id) else {
        return;
    };

    stage_parameter.is_gimmick = param.is_gimmick;
}
