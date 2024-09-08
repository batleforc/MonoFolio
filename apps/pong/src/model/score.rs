use bevy::{
    math::Vec2,
    prelude::{Event, EventReader, EventWriter, Query, ResMut, Resource, With},
    window::Window,
};

use super::{ball::Ball, Position, Scorer, Velocity};

#[derive(Resource, Default)]
pub struct Score {
    pub player1: u32,
    pub player2: u32,
}

#[derive(Event)]
pub struct Scored {
    pub player: Scorer,
}

pub fn detect_scoring(
    ball: Query<&Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single() {
            if ball.0.x < -window_width / 2. {
                events.send(Scored {
                    player: Scorer::Player2,
                });
            } else if ball.0.x > window_width / 2. {
                events.send(Scored {
                    player: Scorer::Player1,
                });
            }
        }
    }
}

pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
            match event.player {
                Scorer::Player1 => {
                    velocity.0 = Vec2::new(-1.0, 1.0);
                }
                Scorer::Player2 => {
                    velocity.0 = Vec2::new(1.0, 1.0);
                }
            }
            position.0 = Vec2::new(0.0, 0.0);
        }
    }
}

pub fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.player {
            Scorer::Player1 => score.player1 += 1,
            Scorer::Player2 => score.player2 += 1,
        }
        println!("Player 1: {}, Player 2: {}", score.player1, score.player2);
    }
}
