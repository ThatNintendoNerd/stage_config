use smash_stage::app::{GlobalStageParameter, SpiritsBattleData};

use crate::{config::Config, offsets::Offsets};

pub fn try_set_stage_additional_settings(stage_parameter: &mut GlobalStageParameter) {
    if is_invalid_melee_mode() {
        return;
    }

    let stage_id = stage_parameter.stage_id();

    if let Some(setting) = Config::get().stage_additional_settings.get(&stage_id) {
        if *setting != 0 {
            let mut spirits_battle_data = SpiritsBattleData::default();

            spirits_battle_data.stage_id = stage_id;
            spirits_battle_data.stage_additional_setting = *setting;

            unsafe {
                set_stage_additional_settings(&spirits_battle_data, stage_parameter);
            }
        }
    }
}

fn is_invalid_melee_mode() -> bool {
    use smash_stage::app::{GlobalParameter, MeleeMode};

    matches!(
        GlobalParameter::global_melee_parameter().melee_mode(),
        MeleeMode::Standard
            | MeleeMode::StandardMulti
            | MeleeMode::SpiritsBattle
            | MeleeMode::SpiritsBattleMulti
    )
}

#[skyline::from_offset(Offsets::get().set_stage_additional_settings)]
fn set_stage_additional_settings(
    spirits_battle_data: &SpiritsBattleData,
    stage_parameter: &mut GlobalStageParameter,
);
