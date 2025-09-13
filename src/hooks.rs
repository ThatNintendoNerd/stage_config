use libc2::app::{GlobalStageParameter, StageBase, StageID};

use crate::{config::Config, offsets::Offsets, stage};

#[skyline::hook(offset = Offsets::get().stage_base_pre_setup)]
fn stage_base_pre_setup(stage_base: &StageBase) {
    original!()(stage_base);

    stage::register_all_dynamic_collision(stage_base);
    stage::set_gravity_param(stage_base.stage_id());
}

#[skyline::hook(offset = Offsets::get().is_flat_stage)]
fn is_flat_stage(stage_id: StageID) -> bool {
    if let Some(value) = Config::get().is_flat_stage.get(&stage_id) {
        return *value;
    }

    original!()(stage_id)
}

#[skyline::hook(offset = Offsets::get().set_stage_random_setting)]
fn set_stage_random_setting(stage_parameter: &mut GlobalStageParameter, seed: u32) {
    stage::set_stage_additional_setting(stage_parameter);
    stage::set_gimmick_param(stage_parameter);

    original!()(stage_parameter, seed);
}

/// Installs all the function hooks.
pub fn install() {
    skyline::install_hooks!(
        stage_base_pre_setup,
        is_flat_stage,
        set_stage_random_setting,
    );
}
