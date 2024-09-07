use bevy::{
    app::{App, Startup, Update},
    prelude::{default, Camera, Camera2dBundle, Commands, IntoSystemConfigs, Query, Transform},
    DefaultPlugins,
};
use pong::model::{
    ball::{move_ball, spawn_ball},
    collision::handle_collisions,
    user_row::spawn_user_row,
    Position,
};

fn add_cam(mut commands: Commands) {
    println!("Spawning Camera");
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    });
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_cam, spawn_ball, spawn_user_row))
        .add_systems(
            Update,
            (
                move_ball,
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}
