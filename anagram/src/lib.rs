use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut answer = HashSet::new();

    let sorted_word = sort_word(word);

    possible_anagrams.iter().enumerate().for_each(|(i, v)| {
        if word.to_string().to_lowercase() == v.to_string().to_lowercase() {
        } else if sort_word(v) == sorted_word {
            answer.insert(possible_anagrams[i]);
        }
    });

    return answer;
}

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
