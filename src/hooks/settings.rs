use smash_stage::app::{MeleeMode, SpiritsBattleData, StageDescription};

use crate::{config::CONFIG, offsets::OFFSETS};

#[skyline::hook(offset = OFFSETS.set_stage_random_settings)]
pub fn set_stage_random_settings(stage_description: &mut StageDescription, seed: u32) {
    if !matches!(
        MeleeMode::instance(),
        MeleeMode::Standard
            | MeleeMode::StandardMulti
            | MeleeMode::SpiritsBattle
            | MeleeMode::SpiritsBattleMulti
    ) {
        let stage_id = stage_description.stage_id();

        for (stage, setting) in CONFIG.stage_additional_settings.iter() {
            if *stage == stage_id && *setting != 0 {
                let mut spirits_battle_data = SpiritsBattleData::default();

                spirits_battle_data.stage_id = stage_id;
                spirits_battle_data.stage_additional_setting = *setting;

                unsafe {
                    set_stage_additional_settings(&spirits_battle_data, stage_description);
                }

                break;
            }
        }
    }

    original!()(stage_description, seed);
}

#[skyline::from_offset(OFFSETS.set_stage_additional_settings)]
fn set_stage_additional_settings(
    spirits_battle_data: &SpiritsBattleData,
    stage_description: &mut StageDescription,
);
