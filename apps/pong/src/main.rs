use bevy::{
    app::{App, Startup, Update},
    prelude::{default, Camera, Camera2dBundle, Commands, IntoSystemConfigs, Query, Transform},
    DefaultPlugins,
};
use pong::model::{
    ball::{move_ball, spawn_ball},
    collision::handle_collisions,
    gutter::spawn_gutters,
    score::{detect_scoring, reset_ball, update_score, Score, Scored},
    scoreboard::{spawn_scoreboard, update_scoreboard},
    user_row::{handle_player_input, move_paddles, spawn_user_row},
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
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                add_cam,
                spawn_ball,
                spawn_user_row,
                spawn_gutters,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                handle_player_input,
                detect_scoring,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(update_score),
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
                move_paddles.after(handle_player_input),
            ),
        )
        .run();
}
