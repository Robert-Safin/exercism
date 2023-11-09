/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if is_valid_input(isbn) {
        if check_sum(isbn) {
            if has_an_x(isbn) {
                x_is_valid_position(isbn)
            } else {
                true
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn is_valid_input(isbn: &str) -> bool {
    let clean: Vec<_> = isbn
        .replace("-", "")
        .chars()
        .map(|char| char == 'X' || char.to_digit(10).is_some())
        .collect();

    if clean.len() != 10 {
        false
    } else if clean.into_iter().any(|v| v == false) {
        false
    } else {
        true
    }
}

fn check_sum(isbn: &str) -> bool {
    let clean: Vec<_> = isbn.replace("-", "").chars().collect();

    let mut sum: u32 = 0;
    let mut count = 10;

    for char in clean {
        if char.to_digit(10).is_some() || char == 'X' {
            let val: u32 = if char == 'X' {
                10
            } else {
                char.to_digit(10).unwrap()
            };
            let val = val * count;
            count -= 1;
            sum += val
        }
    }
    sum % 11 == 0
}

fn has_an_x(isbn: &str) -> bool {
    isbn.replace("-", "").contains('X')
}

fn x_is_valid_position(isbn: &str) -> bool {
    let clean: Vec<_> = isbn.replace("-", "").chars().collect();
    let mut x_index: Option<usize> = None;
    for (i, char) in clean.into_iter().enumerate() {
        if char == 'X' {
            x_index = Some(i);
        }
    }
    if x_index.unwrap() == 9 {
        true
    } else {
        false
    }
}
