use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let mut word_map = HashMap::new();

    let word = word.chars().map(|c| c.to_lowercase());
    for c in word {
        *word_map.entry(c).or_insert(0) += 1;
    }

    for possible_anagram in possible_anagrams {
        let lowercase_anagram = possible_anagram.chars().map(|c| c.to_ascii_lowercase());

        let mut possible_anagram_map = HashMap::new();

        for c in lowercase_anagram {
            *possible_anagram_map.entry(c).or_insert(0) += 1;
        }

        if possible_anagram_map == word_map {
            anagrams.insert(*possible_anagram);
        }
    }

    anagrams
}
