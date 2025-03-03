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
struct Tile {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct TileReference(Option<Entity>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_board, create_letter_tiles))
        .run();
}

fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resolution.set(
        TILE_SIZE * BOARD_SIZE as f32,
        // Area below for two rows of tiles
        TILE_SIZE * BOARD_SIZE as f32 + (TILE_SIZE * 2.0),
    );
    window.title = String::from("Zeeb");
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, -TILE_SIZE, 0.0),
    ));
}

fn draw_board(mut commands: Commands) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let color = if (row + col) % 2 == 0 {
                Color::srgb(0.65, 0.45, 0.25)
            } else {
                Color::srgb(0.9, 0.85, 0.7)
            };

            let x =
                col as f32 * TILE_SIZE - (BOARD_SIZE as f32 * TILE_SIZE) / 2.0 + (TILE_SIZE / 2.0);
            let y =
                row as f32 * TILE_SIZE - (BOARD_SIZE as f32 * TILE_SIZE) / 2.0 + (TILE_SIZE / 2.0);

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
                Tile { x: col, y: row },
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
                    Transform::from_xyz(x, y, 1.0),
                    TileReference(None),
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
