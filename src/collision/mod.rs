use wrapped2d::b2::{Vec2, BodyHandle, MetaBody, World, FixtureHandle};
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2;

pub struct PointCollider2D {
    pub point: Vec2,
    pub body_handle: Option<BodyHandle>,
}

impl PointCollider2D {
    pub fn new() -> PointCollider2D {
        PointCollider2D {
            point: Vec2 {x: 0.0, y: 0.0 },
            body_handle: None,
        }
    }
}


/*
collision_system(#resource PointCollider2D) {
    if collider.check(x, y) {

    }
}
*/