use bevy::{
    asset::Assets,
    color::Color,
    math::Vec2,
    prelude::{Bundle, Commands, Component, Mesh, Query, Rectangle, ResMut},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    window::Window,
};

const USER_ROW_SPEED: f32 = 1.;
const USER_ROW_WIDTH: f32 = 10.;
const USER_ROW_HEIGHT: f32 = 50.;

use super::{Position, Shape, Velocity};

#[derive(Component)]
pub struct UserRow {
    pub name: String,
    pub points: u32,
}

#[derive(Bundle)]
pub struct UserRowBundle {
    pub user_row: UserRow,
    pub position: Position,
    pub shape: Shape,
    pub velocity: Velocity,
}

impl UserRowBundle {
    pub fn new(name: String, x: f32, y: f32) -> Self {
        UserRowBundle {
            user_row: UserRow { name, points: 0 },
            position: Position(Vec2::new(x, y)),
            shape: Shape(Vec2::new(USER_ROW_WIDTH, USER_ROW_HEIGHT)),
            velocity: Velocity(Vec2::new(0., 0.)),
        }
    }
}

pub fn spawn_user_row(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning User Row");
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let mesh = Mesh::from(Rectangle::new(USER_ROW_WIDTH, USER_ROW_HEIGHT));
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            UserRowBundle::new("User1".to_string(), right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: materials.add(ColorMaterial::from(Color::srgb(0., 1., 0.))),
                ..Default::default()
            },
        ));

        commands.spawn((
            UserRowBundle::new("User2".to_string(), left_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(ColorMaterial::from(Color::srgb(0., 0., 1.))),
                ..Default::default()
            },
        ));
    }
}
