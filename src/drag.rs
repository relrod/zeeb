use bevy::prelude::*;

use crate::board_state::BoardState;
use crate::consts::*;
use crate::letter_tile::LetterTile;
use crate::wordlist::WordList;

#[derive(Component)]
pub struct Draggable {
    pub is_dragging: bool,
    pub last_position: Vec2,
    pub is_on_board: bool,
    pub game_start_position: Vec3,
}

pub fn drag_tile(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &mut Draggable, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut board: ResMut<BoardState>,
    q_lettertiles: Query<&LetterTile>,
    // Temporary hack:
    valid_word_list: Res<WordList>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };

    for (entity, mut draggable, mut transform) in query.iter_mut() {
        if mouse_input.pressed(MouseButton::Left) && draggable.is_dragging {
            transform.translation = world_position.extend(transform.translation.z);
            break;
        }

        if mouse_input.just_pressed(MouseButton::Left) {
            let tile_position = transform.translation.xy();
            if tile_position.distance(world_position) < TILE_SIZE / 2.0 {
                draggable.is_dragging = true;
            }
        }

        if mouse_input.just_released(MouseButton::Left) && draggable.is_dragging {
            draggable.is_dragging = false;

            // If we are underneath the board, it's a free-for-all, don't snap to grid
            if world_position.y < -BOARD_CENTER {
                // But we still need to free up the grid cell if the tile was on the board
                if draggable.is_on_board {
                    let (col, row) = BoardState::closest_cell_to_world(draggable.last_position);
                    board.remove_tile(col, row);
                }
                draggable.is_on_board = false;
                // And still keep track of the new "last" position
                draggable.last_position = world_position;
                continue;
            }

            let (col, row) = BoardState::closest_cell_to_world(world_position);

            if board.is_occupied(col, row) {
                // If the cell we're trying to move into is occupied, move the tile back
                transform.translation = draggable.last_position.extend(transform.translation.z);
                // Nothing changed, so don't update the board state.
            } else {
                // Otherwise it's a free cell, so figure out the world position of the cell
                // and move the tile there.
                let cell_center_world = BoardState::world_from_xy(col, row);
                transform.translation = cell_center_world.extend(transform.translation.z);

                // Update internal state so we know if another tile can move here
                board.place_tile(col, row, entity);
                draggable.is_on_board = true;

                // Then free up the old position *if* the old cell is different from the new one
                let (old_col, old_row) = BoardState::closest_cell_to_world(draggable.last_position);
                if old_col != col || old_row != row {
                    board.remove_tile(old_col, old_row);
                }

                // This is what we'll snap the tile back to, if the player tries to
                // move it to an occupied cell in the future. Do this after we free up
                // the old position, since we use the old value to determine where the
                // tile came from.
                draggable.last_position = cell_center_world;
            }

            // Print out the words on the board
            let words = board.words(&q_lettertiles);
            for word in words {
                print!("Word: {}", word);
                if valid_word_list.0.contains(&word) {
                    print!(" - Valid!");
                }
                println!();
            }
        }
    }
}
