// on every year that is evenly divisible by 4
//   except every year that is evenly divisible by 100
//     unless the year is also evenly divisible by 400

pub fn is_leap_year(year: u64) -> bool {
    if is_div_by_4(&year) {
        if is_div_by_100(&year) && !is_div_by_400(&year) {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn is_div_by_4(year: &u64) -> bool {
    if year % 4 == 0 {
        true
    } else {
        false
    }
}

fn is_div_by_100(year: &u64) -> bool {
    if year % 100 == 0 {
        true
    } else {
        false
    }
}

fn is_div_by_400(year: &u64) -> bool {
    if year % 400 == 0 {
        true
    } else {
        false
    }
}
