use libc2::app::StageID;
use semver::Version;
use skyline::{
    hooks::{getRegionAddress, Region},
    patching::Patch,
};
use strum::EnumCount;

use crate::{config::Config, env, offsets::Offsets};

mod instr;
use instr::*;

/// Determines if a stage identifier exists on the given version of the target application.
#[rustfmt::skip]
fn is_valid_stage_id(stage_id: StageID, app_version: &Version) -> bool {
    let app_version_min = match stage_id {
        StageID::Jack_Mementoes
        | StageID::End_Jack_Mementoes
        | StageID::Battle_Jack_Mementoes
        | StageID::ResultStage_Jack => Version::new(3, 0, 0),
        StageID::Brave_Altar
        | StageID::End_Brave_Altar
        | StageID::Battle_Brave_Altar => Version::new(4, 0, 0),
        StageID::Buddy_Spiral
        | StageID::End_Buddy_Spiral
        | StageID::Battle_Buddy_Spiral => Version::new(5, 0, 0),
        StageID::Dolly_Stadium
        | StageID::End_Dolly_Stadium
        | StageID::Battle_Dolly_Stadium => Version::new(6, 0, 0),
        StageID::FE_Shrine
        | StageID::End_FE_Shrine
        | StageID::Battle_FE_Shrine => Version::new(7, 0, 0),
        StageID::Tantan_Spring
        | StageID::End_Tantan_Spring
        | StageID::Battle_Tantan_Spring => Version::new(8, 0, 0),
        StageID::BattleField_S => Version::new(8, 1, 0),
        StageID::Pickel_World
        | StageID::End_Pickel_World
        | StageID::Battle_Pickel_World => Version::new(9, 0, 0),
        StageID::FF_Cave
        | StageID::End_FF_Cave
        | StageID::Battle_FF_Cave
        | StageID::ResultStage_Edge => Version::new(10, 0, 0),
        StageID::Xeno_Alst
        | StageID::End_Xeno_Alst
        | StageID::Battle_Xeno_Alst => Version::new(11, 0, 0),
        StageID::Demon_Dojo
        | StageID::End_Demon_Dojo
        | StageID::Battle_Demon_Dojo => Version::new(12, 0, 0),
        StageID::Trail_Castle
        | StageID::End_Trail_Castle
        | StageID::Battle_Trail_Castle => Version::new(13, 0, 0),
        _ => Version::new(1, 0, 0),
    };

    *app_version >= app_version_min
}

/// Computes the resulting offset from an `ADRP` and an `ADD` instruction, relative to the start of the code segment.
fn calc_offset_from_adrp_add(text: *const u8, adrp_offset: usize, add_offset: usize) -> usize {
    let instr = unsafe { text.add(adrp_offset).cast::<u32>().read() };
    let adrp = Adrp::decode(instr).expect("machine code should represent a valid adrp instruction");

    let instr = unsafe { text.add(add_offset).cast::<u32>().read() };
    let add = AddImm::decode(instr).expect("machine code should represent a valid add instruction");

    (adrp_offset & !0b1111_1111_1111) + adrp.imm as usize + add.imm12 as usize
}

/// Overwrites relative branch offsets for select dynamically constructed stages to assume the default case.
fn patch_create_stage_branch_table() {
    let text = unsafe { getRegionAddress(Region::Text).cast::<u8>() };
    let adrp_offset = Offsets::get().create_stage_branch_table_adrp_instr;
    let add_offset = adrp_offset + size_of::<u32>();
    let branch_table_offset = calc_offset_from_adrp_add(text, adrp_offset, add_offset);
    let branch_table = unsafe {
        &*text
            .add(branch_table_offset)
            .cast::<[i32; StageID::COUNT]>()
    };

    // The Omega form of Peach's Castle is the first stage identifier to use the default case.
    let branch_offset = branch_table[StageID::End_Mario_Castle64 as usize];
    let app_version = env::app_version();

    for stage in Config::get().discard_stage_code.iter().copied() {
        if !is_valid_stage_id(stage, &app_version) {
            eprintln!(
                "[{}] `{stage}` was specified but is not supported on the installed version of the target software.",
                module_path!(),
            );
            continue;
        }

        let offset = branch_table_offset + stage as usize * size_of::<StageID>();

        Patch::in_text(offset).data(branch_offset).unwrap();
    }
}

/// Writes all the memory patches.
pub fn write() {
    patch_create_stage_branch_table();
}
