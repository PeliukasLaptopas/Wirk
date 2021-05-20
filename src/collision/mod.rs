use wrapped2d::b2::{Vec2, BodyHandle};

pub struct PointCollider2D {
    pub point: Vec2,
    pub body: Option<BodyHandle>,
}

impl PointCollider2D {
    pub fn new() -> PointCollider2D {
        PointCollider2D {
            point: Vec2 {x: 0.0, y: 0.0 },
            body: None
        }
    }
}


/*
collision_system(#resource PointCollider2D) {
    if collider.check(x, y) {

    }
}
*/