use bevy::prelude::*;
use rand::seq::IndexedMutRandom;

use crate::board_state::BoardState;
use crate::consts::*;
use crate::drag::Draggable;
use crate::letter_tile::LetterTile;

pub fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resolution.set(
        TILE_SIZE * BOARD_SIZE as f32,
        // Area below for two rows of tiles
        TILE_SIZE * BOARD_SIZE as f32 + (TILE_SIZE * 2.0),
    );
    window.title = String::from("Zeeb");
    commands.spawn((Camera2d, Transform::from_xyz(0.0, -TILE_SIZE, 0.0)));
    commands.insert_resource(BoardState::default());
}

pub fn draw_board(mut commands: Commands) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let color = if (row + col) % 2 == 0 {
                Color::srgb(0.65, 0.45, 0.25)
            } else {
                Color::srgb(0.9, 0.85, 0.7)
            };

            let x = col as f32 * TILE_SIZE - BOARD_CENTER + (TILE_SIZE / 2.0);
            let y = row as f32 * TILE_SIZE - BOARD_CENTER + (TILE_SIZE / 2.0);

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

/// Create 12 [LetterTile]s with "random" letters copied from a game who shall
/// remain un-named.
///
/// We want two rows at the bottom: 10 in the top row and 2 in the bottom row.
// TODO: Move this function to LetterTile
pub fn create_letter_tiles(mut commands: Commands) {
    let mut rng = rand::rng();

    // y-position for the first row, should be immediately below the board
    let first_row = -BOARD_CENTER - TILE_SIZE / 2.0;

    // y-position for the second row, should be below the first row
    let second_row = first_row - TILE_SIZE;

    for (i, row) in LETTER_ROWS.iter().enumerate() {
        if let Some(letter) = row.chars().collect::<Vec<_>>().choose_mut(&mut rng) {
            let x = if i < 10 {
                i as f32 * TILE_SIZE - BOARD_CENTER + TILE_SIZE / 2.0
            } else {
                (i - 10) as f32 * TILE_SIZE - BOARD_CENTER + TILE_SIZE / 2.0
            };

            let y = if i < 10 { first_row } else { second_row };
            commands
                .spawn((
                    Sprite::from_color(Color::srgb(0.75, 0.6, 0.3), Vec2::splat(TILE_SIZE)),
                    Transform::from_xyz(x, y, 100.0),
                    Draggable {
                        is_dragging: false,
                        last_position: Vec2::new(x, y),
                        is_on_board: false,
                        game_start_position: Vec3::new(x, y, 100.0),
                    },
                    LetterTile(letter.to_ascii_lowercase()),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text2d::new(String::from(letter.to_ascii_uppercase())),
                        TextFont { ..default() },
                        TextColor(Color::srgb(0.3, 0.15, 0.15)),
                        Transform::from_translation(Vec3::Z),
                    ));
                });
        }
    }
}
