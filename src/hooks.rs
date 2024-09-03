use smash_stage::app::{GlobalStageParameter, StageBase, StageID};

use crate::{config::Config, offsets::Offsets, service};

#[skyline::hook(offset = Offsets::get().stage_base_pre_setup)]
fn stage_base_pre_setup(stage_base: &StageBase) {
    original!()(stage_base);

    service::try_register_all_dynamic_collisions(stage_base);
    service::try_set_gravity_param(stage_base.stage_id());
}

#[skyline::hook(offset = Offsets::get().is_flat_stage)]
fn is_flat_stage(stage_id: StageID) -> bool {
    if let Some(value) = Config::get().is_flat_stage.get(&stage_id) {
        return *value;
    }

    original!()(stage_id)
}

#[skyline::hook(offset = Offsets::get().set_stage_random_settings)]
fn set_stage_random_settings(stage_parameter: &mut GlobalStageParameter, seed: u32) {
    service::try_set_stage_additional_settings(stage_parameter);
    service::try_set_gimmick_param(stage_parameter);

    original!()(stage_parameter, seed);
}

fn patch_create_stage_branch_table() {
    use skyline::{
        hooks::{getRegionAddress, Region},
        patching::Patch,
    };
    use strum::EnumCount;

    let branch_table_offset = Offsets::get().create_stage_branch_table;
    let branch_table = unsafe {
        &*((getRegionAddress(Region::Text) as usize + branch_table_offset)
            as *const [i32; StageID::COUNT])
    };

    // The Omega form of Peach's Castle is the first stage ID to use the default case.
    let branch_offset = branch_table[StageID::End_Mario_Castle64 as usize];

    for stage in &Config::get().discard_stage_code {
        Patch::in_text(branch_table_offset + (*stage as usize) * std::mem::size_of::<StageID>())
            .data(branch_offset)
            .unwrap();
    }
}

/// Installs all the function hooks and memory patches.
pub fn install() {
    skyline::install_hooks!(
        stage_base_pre_setup,
        is_flat_stage,
        set_stage_random_settings,
    );

    patch_create_stage_branch_table();
}
