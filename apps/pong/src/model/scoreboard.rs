use bevy::{
    color::Color,
    prelude::{default, Commands, Component, DetectChanges, Query, Res, TextBundle, With, Without},
    text::{JustifyText, Text, TextStyle},
    ui::{PositionType, Style, Val},
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
            player_score.sections[0].value = score.player1.to_string();
        }

        if let Ok(mut ai_score) = player2_score.get_single_mut() {
            ai_score.sections[0].value = score.player2.to_string();
        }
    }
}

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a
        // single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts
            // into a `String`, such as `&str`
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        Player1Scoreboard,
    ));

    commands.spawn(
        TextBundle::from_section(
            "Z/S - Player 1",
            TextStyle {
                font_size: 31.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            left: Val::Px(15.0),
            ..default()
        }),
    );

    commands.spawn(
        TextBundle::from_section(
            "UP/DOWN - Player 2",
            TextStyle {
                font_size: 31.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            right: Val::Px(15.0),
            ..default()
        }),
    );

    // Then we do it again for the AI score
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        Player2Scoreboard,
    ));
}
