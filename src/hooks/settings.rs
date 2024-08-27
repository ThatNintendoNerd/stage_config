use smash_stage::app::{GlobalStageParameter, SpiritsBattleData};

use crate::{config::Config, offsets::Offsets};

#[skyline::hook(offset = Offsets::get().set_stage_random_settings)]
pub fn set_stage_random_settings(stage_parameter: &mut GlobalStageParameter, seed: u32) {
    if !is_invalid_melee_mode() {
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

    original!()(stage_parameter, seed);
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
