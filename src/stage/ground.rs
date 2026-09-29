use libc2::app::StageBase;

use crate::config::Config;

/// Registers all specified dynamic collisions based on the working stage identifier.
pub fn register_dynamic_collision(stage_base: &StageBase) {
    let Some(model_names) = Config::get()
        .new_dynamic_collisions
        .get(stage_base.stage_id())
    else {
        return;
    };

    for model_name in model_names.iter().copied() {
        for dynamic_object in stage_base
            .level_data
            .dynamic_object_collection
            .iter()
            .filter(|o| o.name_hash == model_name)
        {
            if stage_base.search_draw_model(model_name).is_none() {
                continue;
            }

            stage_base.create_related_move_floor(dynamic_object);
        }
    }
}
