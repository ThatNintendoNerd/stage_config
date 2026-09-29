use std::mem::MaybeUninit;

use libc2::app::{GlobalStageParameter, SpiritsBattleData};

use crate::{config::Config, offsets::Offsets};

/// Updates the stage's spirit battle settings if the working stage identifier is assigned a specialized setting.
pub fn set_stage_additional_setting(stage_parameter: &mut GlobalStageParameter) {
    if is_invalid_melee_mode() {
        return;
    }

    let stage_id = stage_parameter.stage_id();
    let Some(setting) = Config::get()
        .stage_additional_settings
        .get(stage_id)
        .copied()
        .filter(|s| *s != 0)
    else {
        return;
    };
    let mut spirits_battle_data = MaybeUninit::<SpiritsBattleData>::uninit();

    // SAFETY: The foreign function only reads the fields assigned to.
    unsafe {
        spirits_battle_data.assume_init_mut().stage_id = *stage_id;
        spirits_battle_data
            .assume_init_mut()
            .stage_additional_setting = setting;
    }

    unsafe {
        set_stage_additional_setting_impl(spirits_battle_data.assume_init_ref(), stage_parameter);
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

#[skyline::from_offset(Offsets::get().set_stage_additional_setting)]
fn set_stage_additional_setting_impl(
    spirits_battle_data: &SpiritsBattleData,
    stage_parameter: &mut GlobalStageParameter,
);
