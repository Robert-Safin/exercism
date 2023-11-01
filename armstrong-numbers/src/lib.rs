pub fn is_armstrong_number(num: u32) -> bool {

    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let num_digits = digits.len() as u32;

    let sum: u128 = digits
        .into_iter()
        .map(|digit| (digit as u128).checked_pow(num_digits).unwrap_or(0))
        .sum();

    sum == num as u128

}
