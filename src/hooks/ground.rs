use smash_stage::app::StageBase;

use crate::config::CONFIG;

pub fn register_dynamic_collisions(stage_base: &StageBase) {
    let stage_id = stage_base.stage_description.stage_id();

    for (stage, models) in &CONFIG.new_dynamic_collisions {
        if stage_id == *stage {
            for model_name in models {
                unsafe {
                    for dynamic_object in
                        (*(*stage_base.level_data).dynamic_object_collection).iter()
                    {
                        if (**dynamic_object).name_hash == *model_name
                            && stage_base.search_draw_model(*model_name).is_some()
                        {
                            stage_base.create_model_related_move_floor(&**dynamic_object);
                        }
                    }
                }
            }

            break;
        }
    }
}
