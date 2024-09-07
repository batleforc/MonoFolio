use bevy::{math::Vec2, prelude::Component};

pub mod ball;
pub mod collision;
pub mod user_row;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Shape(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);
