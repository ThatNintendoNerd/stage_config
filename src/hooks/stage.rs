use std::mem;

use skyline::patching::Patch;
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
    for (stage, value) in CONFIG.is_flat_stage.iter() {
        if *stage == stage_id {
            return *value;
        }
    }

    original!()(stage_id)
}

pub fn patch_create_stage_jump_table() {
    for stage in CONFIG.discard_stage_code.iter() {
        Patch::in_text(
            OFFSETS.create_stage_jump_table + (*stage as usize) * mem::size_of::<StageID>(),
        )
        .data(0xFE12DFBC_u32)
        .unwrap();
    }
}
