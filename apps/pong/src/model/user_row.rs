use bevy::{
    asset::Assets,
    color::Color,
    input::ButtonInput,
    math::Vec2,
    prelude::{Bundle, Commands, Component, KeyCode, Mesh, Query, Rectangle, Res, ResMut, With},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    window::Window,
};

const USER_ROW_SPEED: f32 = 1.;
const USER_ROW_WIDTH: f32 = 10.;
const USER_ROW_HEIGHT: f32 = 50.;

use super::{gutter::GUTTER_HEIGHT, Position, Scorer, Shape, Velocity};

#[derive(Component)]
pub struct UserRow {
    pub name: Scorer,
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
    pub fn new(name: Scorer, x: f32, y: f32) -> Self {
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
            UserRowBundle::new(Scorer::Player1, right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: materials.add(ColorMaterial::from(Color::srgb(0., 1., 0.))),
                ..Default::default()
            },
        ));

        commands.spawn((
            UserRowBundle::new(Scorer::Player2, left_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(ColorMaterial::from(Color::srgb(0., 0., 1.))),
                ..Default::default()
            },
        ));
    }
}

pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<(&mut Velocity, &UserRow), With<UserRow>>,
) {
    for (mut velocity, user_row) in &mut paddle {
        if (user_row.name == Scorer::Player1 && keyboard_input.pressed(KeyCode::ArrowUp))
            || (user_row.name == Scorer::Player2 && keyboard_input.pressed(KeyCode::KeyW))
        {
            velocity.0.y = 1.;
        } else if (user_row.name == Scorer::Player1 && keyboard_input.pressed(KeyCode::ArrowDown))
            || (user_row.name == Scorer::Player2 && keyboard_input.pressed(KeyCode::KeyS))
        {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

pub fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<UserRow>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - USER_ROW_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * USER_ROW_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}
