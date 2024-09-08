use std::fmt::Debug;

use bevy::{math::Vec2, prelude::Component};

pub mod ball;
pub mod collision;
pub mod gutter;
pub mod score;
pub mod scoreboard;
pub mod user_row;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Shape(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(PartialEq)]
pub enum Scorer {
    Player1,
    Player2,
}

impl Debug for Scorer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scorer::Player1 => write!(f, "Player 1"),
            Scorer::Player2 => write!(f, "Player 2"),
        }
    }
}
