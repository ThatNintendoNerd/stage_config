use libc2::app::StageID;

use crate::config::ConfigGravityParameter;

/// Updates the parameters for gravity if the given stage identifier is assigned specialized parameters.
pub fn try_set_gravity_param(stage_id: StageID) {
    use crate::config::Config;

    if let Some(param) = Config::get().gravity_param.get(&stage_id) {
        set_gravity_param(param);
    }
}

/// Updates the parameters for gravity.
fn set_gravity_param(param: &ConfigGravityParameter) {
    use smash::app;

    if let Some(instance) = app::BattleObjectWorld::instance_mut() {
        if instance.is_gravity_normal != param.is_gravity_normal {
            instance.is_gravity_normal = param.is_gravity_normal;
        }

        if instance.is_gravity_normal {
            return;
        }

        if let Some(pos) = &param.pos {
            instance.gravity_pos.vec[0] = pos.x;
            instance.gravity_pos.vec[1] = pos.y;
        }
    }
}
