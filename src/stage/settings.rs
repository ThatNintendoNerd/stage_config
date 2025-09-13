use libc2::app::{GlobalStageParameter, SpiritsBattleData};

use crate::{config::Config, offsets::Offsets};

/// Updates the stage's spirit battle settings if the working stage identifier is assigned a specialized setting.
pub fn try_set_stage_additional_settings(stage_parameter: &mut GlobalStageParameter) {
    if is_invalid_melee_mode() {
        return;
    }

    let stage_id = stage_parameter.stage_id();

    if let Some(setting) = Config::get()
        .stage_additional_settings
        .get(&stage_id)
        .copied()
        .filter(|s| *s != 0)
    {
        set_stage_additional_settings(stage_parameter, setting);
    }
}

/// Updates the stage's spirit battle settings.
fn set_stage_additional_settings(stage_parameter: &mut GlobalStageParameter, setting: i8) {
    let mut spirits_battle_data = SpiritsBattleData::default();

    spirits_battle_data.stage_id = stage_parameter.stage_id();
    spirits_battle_data.stage_additional_setting = setting;

    unsafe {
        set_stage_additional_settings_impl(&spirits_battle_data, stage_parameter);
    }
}

/// Returns `true` if the working game mode is unsupported by this feature.
fn is_invalid_melee_mode() -> bool {
    use libc2::app::{GlobalParameter, MeleeMode};

    matches!(
        GlobalParameter::global_melee_parameter().melee_mode(),
        MeleeMode::Standard
            | MeleeMode::StandardMulti
            | MeleeMode::SpiritsBattle
            | MeleeMode::SpiritsBattleMulti
    )
}

#[skyline::from_offset(Offsets::get().set_stage_additional_settings)]
fn set_stage_additional_settings_impl(
    spirits_battle_data: &SpiritsBattleData,
    stage_parameter: &mut GlobalStageParameter,
);
