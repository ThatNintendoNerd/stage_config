use std::collections::HashSet;

use hash40::Hash40;
use smash_stage::app::{StageBase, StageDynamicObject};

pub fn try_register_all_dynamic_collisions(stage_base: &StageBase) {
    use crate::config::Config;

    if let Some(model_names) = Config::get()
        .new_dynamic_collisions
        .get(&stage_base.stage_id())
    {
        register_all_dynamic_collisions(stage_base, model_names);
    }
}

pub fn register_all_dynamic_collisions(stage_base: &StageBase, model_names: &HashSet<Hash40>) {
    for model_name in model_names.iter().copied() {
        unsafe {
            for dynamic_object in (*(*stage_base.level_data).dynamic_object_collection).iter() {
                try_register_dynamic_collision(stage_base, &**dynamic_object, model_name);
            }
        }
    }
}

pub fn try_register_dynamic_collision(
    stage_base: &StageBase,
    dynamic_object: &StageDynamicObject,
    model_name: Hash40,
) {
    if dynamic_object.name_hash != model_name || stage_base.search_draw_model(model_name).is_none()
    {
        return;
    }

    stage_base.create_model_related_move_floor(dynamic_object);
}
