use libc2::app::GlobalStageParameter;

use crate::config::Config;

/// Updates the parameters for stage hazards if the working stage identifier is assigned specialized parameters.
pub fn set_gimmick_param(stage_parameter: &mut GlobalStageParameter) {
    let stage_id = stage_parameter.stage_id();
    let Some(param) = Config::get().gimmick_param.get(&stage_id) else {
        return;
    };

    stage_parameter.is_gimmick = param.is_gimmick;
}
