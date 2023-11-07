pub fn build_proverb(list: &[&str]) -> String {
    let mut answer: String = String::new();

    if list.is_empty() {
        return answer;
    }

    for (i, &word) in list.iter().enumerate() {
        if i + 1 < list.len() {
            let fmt: String = format!("For want of a {} the {} was lost.\n", word, list[i + 1]);
            answer.push_str(&fmt);
        } else {
            let fmt: String = format!("And all for the want of a {}.", list[0]);
            answer.push_str(&fmt);
        }
    }

    answer
}
