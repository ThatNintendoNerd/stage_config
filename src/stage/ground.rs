use hash40::Hash40;
use libc2::app::{StageBase, StageDynamicObject};

use crate::config::Config;

/// Registers all the dynamic collisions if the working stage identifier is assigned a set of model names.
pub fn register_all_dynamic_collision(stage_base: &StageBase) {
    let Some(model_names) = Config::get()
        .new_dynamic_collisions
        .get(&stage_base.stage_id())
    else {
        return;
    };

    for model_name in model_names.iter().copied() {
        unsafe {
            for dynamic_object in (*(*stage_base.level_data).dynamic_object_collection)
                .iter()
                .map(|o| &**o)
            {
                register_dynamic_collision(stage_base, dynamic_object, model_name);
            }
        }
    }
}

/// Registers a dynamic collision if the object is associated with the given model name and the model exists.
fn register_dynamic_collision(
    stage_base: &StageBase,
    dynamic_object: &StageDynamicObject,
    model_name: Hash40,
) {
    if dynamic_object.name_hash != model_name {
        return;
    }

    if stage_base.search_draw_model(model_name).is_none() {
        return;
    }

    stage_base.create_model_related_move_floor(dynamic_object);
}
