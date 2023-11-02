use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut ans: HashMap<char, usize> = HashMap::new();

    for text in input {
        text.to_lowercase().chars().for_each(|v| {
            if v.is_alphabetic() {
                *ans.entry(v).or_insert(0) += 1;
            }
        });
    }

    return ans;
}
