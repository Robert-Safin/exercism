use std::ops::RangeInclusive;

const BOARD: RangeInclusive<u32> = 1..=64;

pub fn square(s: u32) -> u64 {
    if !BOARD.contains(&s) {
        panic!("Square must be between 1 and 64")
    }

    2_u64.pow(s - 1) as u64
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;

    for s in BOARD {
        let grains: u64 = square(s);
        sum += grains
    }
    sum
}
