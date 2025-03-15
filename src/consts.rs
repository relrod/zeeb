pub const TILE_SIZE: f32 = 50.0;
pub const BOARD_SIZE: usize = 10;
pub const BOARD_CENTER: f32 = (BOARD_SIZE as f32 * TILE_SIZE) / 2.0;

/// Letter possibilities for each [`LetterTile`] (one of each group).
///
/// Taken from a game who shall not be named.
pub const LETTER_ROWS: [&str; 12] = [
    "MMLLBY", "VFGKPP", "HHNNRR", "DFRLLW", "RRDLGG", "XKBSZN", "WHHTTP", "CCBTJD", "CCMTTS",
    "OIINNY", "AEIOUU", "AAEEOO",
];
