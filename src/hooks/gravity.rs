use serde::Deserialize;
use smash_stage::app::StageID;

use crate::config::CONFIG;

/// Parameters for gravity.
#[derive(Deserialize)]
pub struct GravityParam {
    /// Boolean flag determining if the stage assumes a flat gravitational plane.
    #[serde(default)]
    is_gravity_normal: bool,

    /// Center position of gravity.
    #[serde(default)]
    pos: Option<GravityCenter>,
}

/// Center position of gravity.
#[derive(Deserialize)]
struct GravityCenter {
    /// Position along the x-axis.
    #[serde(default)]
    x: f32,

    /// Position along the y-axis.
    #[serde(default)]
    y: f32,
}

pub fn set_gravity_param(stage_id: StageID) {
    use smash::app;

    if let Some(param) = CONFIG.gravity_param.get(&stage_id) {
        if let Some(instance) = app::BattleObjectWorld::instance_mut() {
            if instance.is_gravity_normal != param.is_gravity_normal {
                instance.is_gravity_normal = param.is_gravity_normal;
            }

            if !instance.is_gravity_normal {
                if let Some(pos) = &param.pos {
                    instance.gravity_pos.x = pos.x;
                    instance.gravity_pos.y = pos.y;
                }
            }
        }
    }
}
