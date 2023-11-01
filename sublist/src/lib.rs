#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.is_empty() {
        Comparison::Sublist
    } else if second_list.is_empty() {
        Comparison::Superlist
    } else if is_superlist(first_list, second_list) {
        Comparison::Superlist
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    for window in first_list.windows(second_list.len()) {
        if window == second_list {
            return true;
        }
    }

    false
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    for window in second_list.windows(first_list.len()) {
        if window == first_list {
            return true;
        }
    }

    false
}
