use libc2::app::GlobalStageParameter;

use crate::config::Config;

/// Updates the parameters for stage hazards if the given stage identifier is assigned specialized parameters.
pub fn try_set_gimmick_param(stage_parameter: &mut GlobalStageParameter) {
    let stage_id = stage_parameter.stage_id();

    if let Some(param) = Config::get().gimmick_param.get(&stage_id) {
        stage_parameter.is_gimmick = param.is_gimmick;
    }
}
