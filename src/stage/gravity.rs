use libc2::app::StageID;
use smash::app::BattleObjectWorld;

use crate::config::Config;

/// Updates the parameters for gravity if the given stage identifier is assigned specialized parameters.
pub fn set_gravity_param(stage_id: StageID) {
    let Some(param) = Config::get().gravity_param.get(&stage_id) else {
        return;
    };
    let Some(instance) = BattleObjectWorld::instance_mut() else {
        return;
    };

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
