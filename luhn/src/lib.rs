/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned_inputs: Vec<char> = clean_input(code);
    if cleaned_inputs.len() <= 1 {
        false
    } else if has_invalid_chars(code) {
        false
    } else {
        validate_digits(&cleaned_inputs)
    }
}

fn validate_digits(digits: &Vec<char>) -> bool {
    let mut sum: u32 = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        let mut val: u32 = digit.to_digit(10).unwrap();
        if i % 2 != 0 {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
    }
    sum % 10 == 0
}

fn clean_input(input: &str) -> Vec<char> {
    input.chars().filter(|c: &char| c.is_digit(10)).collect()
}

fn has_invalid_chars(input: &str) -> bool {
    input.chars().any(|v: char| !v.is_digit(10) && !v.is_whitespace())
}
