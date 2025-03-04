use bevy::prelude::*;
use rand::seq::IndexedMutRandom;

const TILE_SIZE: f32 = 50.0;
const BOARD_SIZE: usize = 10;
const BOARD_CENTER: f32 = (BOARD_SIZE as f32 * TILE_SIZE) / 2.0;

/// Letter possibilities for each [LetterTile] (one of each group).
///
/// Taken from a game who shall not be named.
const LETTER_ROWS: [&str; 12] = [
    "MMLLBY", "VFGKPP", "HHNNRR", "DFRLLW", "RRDLGG", "XKBSZN", "WHHTTP", "CCBTJD", "CCMTTS",
    "OIINNY", "AEIOUU", "AAEEOO",
];

#[derive(Component)]
struct LetterTile;

#[derive(Component)]
struct Draggable {
    is_dragging: bool,
    last_grid_position: Vec2,
    is_on_board: bool,
    game_start_position: Vec2,
}

#[derive(Resource)]
struct BoardState {
    grid: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            grid: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl BoardState {
    fn is_occupied(&self, col: usize, row: usize) -> bool {
        self.grid[row][col].is_some()
    }

    fn place_tile(&mut self, col: usize, row: usize, entity: Entity) {
        self.grid[row][col] = Some(entity);
    }

    fn remove_tile(&mut self, col: usize, row: usize) {
        self.grid[row][col] = None;
    }

    fn world_from_xy(x: usize, y: usize) -> Vec2 {
        let x = x as f32 * TILE_SIZE - BOARD_CENTER + TILE_SIZE / 2.0;
        let y = -(y as f32 * TILE_SIZE - BOARD_CENTER + TILE_SIZE / 2.0);
        Vec2::new(x, y)
    }

    /// Take a distance of centroids to determine the closest grid square to the
    /// given world point. This iterates the whole board space, but at only 100
    /// cells it's not a big deal. It gets called when the player *stops*
    /// clicking after a drag.
    fn closest_cell_to_world(world_pt: Vec2) -> (usize, usize) {
        let mut min_distance = f32::MAX;
        let mut closest_cell = (0, 0);
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                // Center of the cell
                let world_cell = BoardState::world_from_xy(col, row);
                let distance = world_cell.distance(world_pt);
                if distance < min_distance {
                    min_distance = distance;
                    closest_cell = (col, row);
                }
            }
        }
        closest_cell
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_board, create_letter_tiles))
        .add_systems(Update, (drag_tile, reset_tiles))
        .run();
}

fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resolution.set(
        TILE_SIZE * BOARD_SIZE as f32,
        // Area below for two rows of tiles
        TILE_SIZE * BOARD_SIZE as f32 + (TILE_SIZE * 2.0),
    );
    window.title = String::from("Zeeb");
    commands.spawn((Camera2d, Transform::from_xyz(0.0, -TILE_SIZE, 0.0)));
    commands.insert_resource(BoardState::default());
}

fn draw_board(mut commands: Commands) {
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
fn create_letter_tiles(mut commands: Commands) {
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
                    Transform::from_xyz(x, y, 3.0),
                    Draggable {
                        is_dragging: false,
                        last_grid_position: Vec2::new(x, y),
                        is_on_board: false,
                        game_start_position: Vec2::new(x, y),
                    },
                    LetterTile,
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

fn drag_tile(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &mut Draggable, &mut Transform)>,
    mut events: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut board: ResMut<BoardState>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    for event in events.read() {
        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.position)
        else {
            continue;
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
                        let (col, row) =
                            BoardState::closest_cell_to_world(draggable.last_grid_position);
                        board.remove_tile(col, row);
                    }
                    draggable.is_on_board = false;
                    continue;
                }

                let (col, row) = BoardState::closest_cell_to_world(world_position);

                if board.is_occupied(col, row) {
                    // If the cell we're trying to move into is occupied, move the tile back
                    transform.translation =
                        draggable.last_grid_position.extend(transform.translation.z);
                    // Nothing changed, so don't update the board state.
                } else {
                    // Otherwise it's a free cell, so figure out the world position of the cell
                    // and move the tile there.
                    let cell_center_world = BoardState::world_from_xy(col, row);
                    transform.translation = cell_center_world.extend(transform.translation.z);

                    // Update internal state so we know if another tile can move here
                    board.place_tile(col, row, entity);
                    draggable.is_on_board = true;

                    // Then free up the old position
                    let (old_col, old_row) =
                        BoardState::closest_cell_to_world(draggable.last_grid_position);
                    board.remove_tile(old_col, old_row);

                    // This is what we'll snap the tile back to, if the player tries to
                    // move it to an occupied cell in the future. Do this after we free up
                    // the old position, since we use the old value to determine where the
                    // tile came from.
                    draggable.last_grid_position = cell_center_world;
                }
            }
        }
    }
}

fn reset_tiles(
    mut query: Query<(&mut Draggable, &mut Transform), With<LetterTile>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (mut draggable, mut transform) in query.iter_mut() {
            draggable.is_dragging = false; // Might be mid-drag when we reset
            transform.translation = draggable.game_start_position.extend(0.0);
        }
    }
}
