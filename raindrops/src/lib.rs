
pub fn raindrops(n: u32) -> String {
    let mut answer = Vec::new();
    match n {
        n if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 => answer.push("PlingPlangPlong".to_string()),
        n if n % 3 == 0 && n % 5 == 0 => answer.push("PlingPlang".to_string()),
        n if n % 3 == 0 && n % 7 == 0 => answer.push("PlingPlong".to_string()),
        n if n % 5 == 0 && n % 7 == 0 => answer.push("PlangPlong".to_string()),
        n if n % 3 == 0 => answer.push("Pling".to_string()),
        n if n % 5 == 0 => answer.push("Plang".to_string()),
        n if n % 7 == 0 => answer.push("Plong".to_string()),
        n => answer.push(n.to_string()),
    };
    return answer.join("");
}
