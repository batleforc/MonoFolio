use bevy::{
    asset::Assets,
    color::Color,
    math::Vec2,
    prelude::{Bundle, Circle, Commands, Component, Mesh, Mesh2d, Query, ResMut, With},
    sprite::{ColorMaterial, MeshMaterial2d},
};

use super::{Position, Shape, Velocity};

const BALL_SIZE: f32 = 5.;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub shape: Shape,
    pub position: Position,
    pub velocity: Velocity,
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> Self {
        BallBundle {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0.0, 0.0)),
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning Ball");

    let mesh = Mesh::from(Circle::new(BALL_SIZE));
    let material = ColorMaterial::from(Color::srgb(1.0, 0., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        BallBundle::new(1., 1.),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
    ));
}

pub fn move_ball(
    // Give me all positions that also contain a `Ball` component
    mut ball: Query<(&mut Position, &Velocity), With<Ball>>,
) {
    if let Ok((mut transform, velocity)) = ball.get_single_mut() {
        transform.0 += velocity.0;
    }
}
