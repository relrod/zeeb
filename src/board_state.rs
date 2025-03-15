use bevy::prelude::*;

use crate::consts::{BOARD_CENTER, BOARD_SIZE, TILE_SIZE};
use crate::letter_tile::LetterTile;

#[derive(Resource)]
pub struct BoardState {
    pub grid: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            grid: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl BoardState {
    pub fn is_occupied(&self, col: usize, row: usize) -> bool {
        self.grid[row][col].is_some()
    }

    pub fn place_tile(&mut self, col: usize, row: usize, entity: Entity) {
        self.grid[row][col] = Some(entity);
    }

    pub fn remove_tile(&mut self, col: usize, row: usize) {
        self.grid[row][col] = None;
    }

    pub fn reset(&mut self) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                self.grid[row][col] = None;
            }
        }
    }

    pub fn world_from_xy(x: usize, y: usize) -> Vec2 {
        let x = x as f32 * TILE_SIZE - BOARD_CENTER + TILE_SIZE / 2.0;
        let y = -(y as f32 * TILE_SIZE - BOARD_CENTER + TILE_SIZE / 2.0);
        Vec2::new(x, y)
    }

    /// Take a distance of centroids to determine the closest grid square to the
    /// given world point. This iterates the whole board space, but at only 100
    /// cells it's not a big deal. It gets called when the player *stops*
    /// clicking after a drag.
    pub fn closest_cell_to_world(world_pt: Vec2) -> (usize, usize) {
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

    /// Find a tile. Any tile. One from which to start a traversal of the board.
    ///
    /// There is surely a more efficient way to do this (probably caching where
    /// a tile is placed at placement time and using that and keeping it
    /// updated), but right now this just scans the board and looks for the
    /// first thing.
    pub fn first_tile(&self) -> Option<(usize, usize)> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let Some(_) = self.grid[row][col] else {
                    continue;
                };
                return Some((col, row));
            }
        }
        None
    }

    /// Determine if the board is contiguous, i.e. all tiles are connected to
    /// each other. This is a simple flood-fill algorithm (DFS) that starts at
    /// the first tile it finds and marks all connected tiles as visited.
    #[allow(dead_code)]
    pub fn is_contiguous(&self) -> bool {
        let mut visited = [[false; BOARD_SIZE]; BOARD_SIZE];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let Some(start) = self.first_tile() else {
            // If no tile is on the board, we are trivially connected.
            return true;
        };
        stack.push(start);
        while let Some((col, row)) = stack.pop() {
            if visited[row][col] || self.grid[row][col].is_none() {
                continue;
            }

            visited[row][col] = true;

            // Check up, down, left, right
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let row = i8::try_from(row).expect("row is in bounds");
                let col = i8::try_from(col).expect("col is in bounds");
                let neighbor_row = row + dr;
                let neighbor_col = col + dc;
                // Stay in bounds, especially since we cast away from usize above
                if neighbor_row < 0 || neighbor_col < 0 {
                    continue;
                }
                // We know they're >= 0 now, so just shadow them... and shut clippy up
                #[allow(clippy::cast_sign_loss)]
                let neighbor_row = neighbor_row as usize;

                #[allow(clippy::cast_sign_loss)]
                let neighbor_col = neighbor_col as usize;

                if neighbor_row >= BOARD_SIZE || neighbor_col >= BOARD_SIZE {
                    continue;
                }
                // We're in bounds; check the cell
                if self.grid[neighbor_row][neighbor_col].is_some() {
                    stack.push((neighbor_col, neighbor_row));
                }
            }
        }

        #[allow(clippy::needless_range_loop)]
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.grid[row][col].is_some() && !visited[row][col] {
                    return false;
                }
            }
        }
        true
    }

    /// Collect collections of letters ("words") from the board.
    ///
    /// "words" is quoted because we collect any collection of letters greater
    /// than length 1.
    ///
    /// As a micro-optimization, we could stop and return early if we come
    /// across any two letter words, as those are invalid in Zeeb. But we don't
    /// do that here. We just collect all "words" of length >= 2.
    pub fn words(&self, query: &Query<&LetterTile>) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut current_word: String = String::new();

        // For every column, scan down and look for words
        for col in 0..BOARD_SIZE {
            for row in 0..BOARD_SIZE {
                let Some(tile) = self.grid[row][col] else {
                    if current_word.len() > 1 {
                        words.push(current_word.clone());
                    }
                    current_word.clear();
                    continue;
                };

                let Ok(LetterTile(letter)) = query.get(tile) else {
                    continue;
                };
                current_word.push(*letter);
            }

            // If we're at the end of the column and have a word, add it
            if current_word.len() > 1 {
                words.push(current_word.clone());
            }
            current_word.clear();
        }

        // Now the same for rows
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let Some(tile) = self.grid[row][col] else {
                    if current_word.len() > 1 {
                        words.push(current_word.clone());
                    }
                    current_word.clear();
                    continue;
                };

                let Ok(LetterTile(letter)) = query.get(tile) else {
                    continue;
                };
                current_word.push(*letter);
            }

            // If we're at the end of the row and have a word, add it
            if current_word.len() > 1 {
                words.push(current_word.clone());
            }
            current_word.clear();
        }

        words
    }
}
