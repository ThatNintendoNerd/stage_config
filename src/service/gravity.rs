use serde::Deserialize;
use smash_stage::app::StageID;

/// The parameters for gravity.
#[derive(Deserialize)]
pub struct GravityParam {
    /// Determines if the stage should assume a flat gravitational plane.
    #[serde(default)]
    is_gravity_normal: bool,

    /// The position of the gravitational force.
    #[serde(default)]
    pos: Option<GravityPos>,
}

/// The position of the gravitational force.
#[derive(Deserialize)]
struct GravityPos {
    /// The position along the x-axis.
    #[serde(default)]
    x: f32,

    /// The position along the y-axis.
    #[serde(default)]
    y: f32,
}

/// Updates the parameters for gravity if the given stage identifier is assigned specialized parameters.
pub fn try_set_gravity_param(stage_id: StageID) {
    use crate::config::Config;

    if let Some(param) = Config::get().gravity_param.get(&stage_id) {
        set_gravity_param(param);
    }
}

/// Updates the parameters for gravity.
fn set_gravity_param(param: &GravityParam) {
    use smash::app;

    if let Some(instance) = app::BattleObjectWorld::instance_mut() {
        if instance.is_gravity_normal != param.is_gravity_normal {
            instance.is_gravity_normal = param.is_gravity_normal;
        }

        if instance.is_gravity_normal {
            return;
        }

        if let Some(pos) = &param.pos {
            instance.gravity_pos.x = pos.x;
            instance.gravity_pos.y = pos.y;
        }
    }
}
