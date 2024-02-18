use smash_stage::app::{StageBase, StageID};

use crate::{config::CONFIG, hooks, offsets::OFFSETS};

#[skyline::hook(offset = OFFSETS.stage_base_pre_setup)]
pub fn stage_base_pre_setup(stage_base: &StageBase) {
    original!()(stage_base);

    hooks::ground::register_dynamic_collisions(stage_base);
    hooks::gravity::set_gravity_param(stage_base.stage_description.stage_id());
}

#[skyline::hook(offset = OFFSETS.is_flat_stage)]
pub fn is_flat_stage(stage_id: StageID) -> bool {
    for (stage, value) in &CONFIG.is_flat_stage {
        if *stage == stage_id {
            return *value;
        }
    }

    original!()(stage_id)
}

pub fn patch_create_stage_jump_table() {
    use skyline::{
        hooks::{getRegionAddress, Region},
        patching::Patch,
    };
    use strum::EnumCount;

    let jump_table = unsafe {
        &*((getRegionAddress(Region::Text) as usize + OFFSETS.create_stage_jump_table)
            as *const [i32; StageID::COUNT])
    };

    // The Omega form of Peach's Castle is the first stage ID to use the default case.
    let jump_offset = jump_table[StageID::End_Mario_Castle64 as usize];

    for stage in &CONFIG.discard_stage_code {
        Patch::in_text(
            OFFSETS.create_stage_jump_table + (*stage as usize) * std::mem::size_of::<StageID>(),
        )
        .data(jump_offset)
        .unwrap();
    }
}
