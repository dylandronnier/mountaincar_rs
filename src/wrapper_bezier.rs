use crate::mountaincar;
use bevy::{math::cubic_splines::CubicCurve, prelude::*};

// impl CubicTransform for Transform {
//     fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform {
//         let p = c.position(pos);
//         let dp = c.velocity(pos).normalize();
//         Transform::from_xyz(p.x - PADDING * dp.y, p.y + PADDING * dp.x, z_coordinate)
//             .with_rotation(Quat::from_rotation_z(f32::atan(dp.y / dp.x)))
//     }
// }

#[derive(Resource)]
pub struct Wrapper {
    pub m: mountaincar::MountainCar<CubicCurve<Vec2>>,
}

impl mountaincar::Ground for CubicCurve<Vec2> {
    fn slope(&self, x: f32) -> f32 {
        let v = self.velocity(x);
        v.y / v.x
    }
}
