use smash_stage::app::{StageBase, StageID};

use crate::{config::Config, hooks, offsets::Offsets};

#[skyline::hook(offset = Offsets::get().stage_base_pre_setup)]
pub fn stage_base_pre_setup(stage_base: &StageBase) {
    original!()(stage_base);

    hooks::ground::register_dynamic_collisions(stage_base);
    hooks::gravity::set_gravity_param(stage_base.stage_id());
}

#[skyline::hook(offset = Offsets::get().is_flat_stage)]
pub fn is_flat_stage(stage_id: StageID) -> bool {
    if let Some(value) = Config::get().is_flat_stage.get(&stage_id) {
        return *value;
    }

    original!()(stage_id)
}

pub fn patch_create_stage_jump_table() {
    use skyline::{
        hooks::{getRegionAddress, Region},
        patching::Patch,
    };
    use strum::EnumCount;

    let jump_table_offset = Offsets::get().create_stage_jump_table;
    let jump_table = unsafe {
        &*((getRegionAddress(Region::Text) as usize + jump_table_offset)
            as *const [i32; StageID::COUNT])
    };

    // The Omega form of Peach's Castle is the first stage ID to use the default case.
    let jump_offset = jump_table[StageID::End_Mario_Castle64 as usize];

    for stage in &Config::get().discard_stage_code {
        Patch::in_text(jump_table_offset + (*stage as usize) * std::mem::size_of::<StageID>())
            .data(jump_offset)
            .unwrap();
    }
}
