use smash_stage::app::StageBase;

use crate::config::CONFIG;

pub fn register_dynamic_collisions(stage_base: &StageBase) {
    let Some(models) = CONFIG.new_dynamic_collisions.get(&stage_base.stage_id()) else {
        return;
    };

    for model_name in models {
        unsafe {
            for dynamic_object in (*(*stage_base.level_data).dynamic_object_collection).iter() {
                if (**dynamic_object).name_hash == *model_name
                    && stage_base.search_draw_model(*model_name).is_some()
                {
                    stage_base.create_model_related_move_floor(&**dynamic_object);
                }
            }
        }
    }
}
