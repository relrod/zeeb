/// Functionality for determining the list of valid words.
use bevy::prelude::*;
use std::collections::HashSet;

use crate::letter_tile::LetterTile;

#[derive(Resource)]
pub struct WordList(pub HashSet<String>);

/// Use `/usr/share/dict/words` to determine the list of valid words given the
/// letters in play and store them as a resource. Runs as a Startup system.
pub fn determine_valid_words(mut commands: Commands, query: Query<&LetterTile>) {
    let letters: Vec<char> = query.iter().map(|tile| tile.0).collect();
    let word_list_str = include_str!("../assets/american-english-huge");

    let word_list_set = word_list_str
        .lines()
        .filter(|word| word.chars().all(|c| letters.contains(&c)))
        .filter(|word| word.len() >= 3)
        .map(String::from)
        .collect();

    let word_list = WordList(word_list_set);
    commands.insert_resource(word_list);
}
