use bevy::{
    color::Color,
    prelude::{default, Commands, Component, DetectChanges, Query, Res, Text, With, Without},
    text::{JustifyText, TextColor, TextFont, TextLayout},
    ui::{Node, PositionType, Val},
};

use super::score::Score;

#[derive(Component)]
pub struct Player1Scoreboard;

#[derive(Component)]
pub struct Player2Scoreboard;

pub fn update_scoreboard(
    mut player_score: Query<&mut Text, With<Player1Scoreboard>>,
    mut player2_score: Query<&mut Text, (With<Player2Scoreboard>, Without<Player1Scoreboard>)>, // nécessaire sinon conflit avec la ligne au dessus
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.0 = score.player1.to_string();
        }

        if let Ok(mut ai_score) = player2_score.get_single_mut() {
            ai_score.0 = score.player2.to_string();
        }
    }
}

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        Text::new("0"),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        Player1Scoreboard,
    ));

    commands.spawn((
        Text::new("Z/S - Player 1"),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 31.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("UP/DOWN - Player 2"),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 31.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            right: Val::Px(15.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("0"),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        Player2Scoreboard,
    ));
}
