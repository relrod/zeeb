#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::needless_pass_by_value)]

mod board_state;
mod consts;
mod drag;
mod letter_tile;
mod startup;
mod wordlist;

use bevy::prelude::*;

use crate::board_state::BoardState;
use crate::drag::{Draggable, drag_tile};
use crate::letter_tile::LetterTile;
use crate::startup::{create_letter_tiles, draw_board, setup};
use crate::wordlist::determine_valid_words;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup,
                draw_board,
                (create_letter_tiles, determine_valid_words).chain(),
            ),
        )
        .add_systems(Update, (drag_tile, reset_tiles))
        .run();
}

fn reset_tiles(
    mut query: Query<(&mut Draggable, &mut Transform), With<LetterTile>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<BoardState>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (mut draggable, mut transform) in &mut query {
            draggable.is_dragging = false; // Might be mid-drag when we reset
            transform.translation = draggable.game_start_position;
            draggable.last_position = transform.translation.xy();
            draggable.is_on_board = false;
        }
        board.reset();
    }
}
