use libc2::app::StageID;
use skyline::{
    hooks::{getRegionAddress, Region},
    patching::Patch,
};
use strum::EnumCount;

use crate::{config::Config, offsets::Offsets};

/// Overwrites relative branch offsets for select dynamically constructed stages to assume the default case.
fn patch_create_stage_branch_table() {
    let branch_table_offset = Offsets::get().create_stage_branch_table;
    let branch_table = unsafe {
        &*((getRegionAddress(Region::Text) as usize + branch_table_offset)
            as *const [i32; StageID::COUNT])
    };

    // The Omega form of Peach's Castle is the first stage identifier to use the default case.
    let branch_offset = branch_table[StageID::End_Mario_Castle64 as usize];

    for stage in Config::get().discard_stage_code.iter().copied() {
        let offset = branch_table_offset + (stage as usize) * size_of::<StageID>();

        Patch::in_text(offset).data(branch_offset).unwrap();
    }
}

/// Writes all the memory patches.
pub fn write() {
    patch_create_stage_branch_table();
}
