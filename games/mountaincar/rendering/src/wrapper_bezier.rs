use crate::{HEIGHT, WIDTH};
use bevy::reflect::List;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{math::cubic_splines::CubicCurve, prelude::*, render::mesh::PrimitiveTopology};
use mountaincar_env::{Ground, MountainCar};
use std::ops::{Add, Div};

pub struct RockyRoad(pub CubicCurve<Vec2>);

const PADDING: f32 = 13.0;

#[derive(Resource)]
pub struct Wrapper {
    pub m: MountainCar<RockyRoad>,
}

impl Ground for RockyRoad {
    fn slope(&self, x: f32) -> f32 {
        let v = self.0.velocity(x);
        v.y / v.x
    }

    fn derivivative(&self, x: f32) -> f32 {
        1_400.0 * self.0.velocity(x).length_recip()
    }
}

#[derive(Debug, Clone)]
pub struct TriangleStrip {
    pub points: Vec<Vec3>,
}

pub trait CubicTransform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform;
}

impl From<TriangleStrip> for Mesh {
    fn from(line: TriangleStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let v = Vec2::new(WIDTH, HEIGHT);
        let mut line_points2d: Vec<Vec2> = Vec::new();
        for p in line.points.iter() {
            line_points2d.push(
                <Vec3 as FromReflect>::from_reflect(p)
                    .unwrap()
                    .xy()
                    .add(v.div(2.0))
                    .div(v),
            );
        }
        Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, line_points2d)
    }
}

impl CubicTransform for Transform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform {
        let p = c.position(pos);
        let dp = c.velocity(pos);
        let lambda = dp.distance(Vec2::ZERO);
        Transform::from_xyz(
            p.x - PADDING * dp.y / lambda.powf(0.9),
            p.y + PADDING * dp.x / lambda.powf(0.9),
            z_coordinate,
        )
        .with_rotation(Quat::from_rotation_z(f32::atan(dp.y / dp.x)))
    }
}
