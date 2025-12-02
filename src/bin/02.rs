use std::ops::RangeInclusive;

use advent_of_code::arrayvec::ArrayVec;
use memchr::memmem;

advent_of_code::solution!(2);

fn input_iter(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.trim().split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        let [start, end] = [start, end].map(|n| n.parse().unwrap());
        start..=end
    })
}

const MAX_DIGITS: usize = (u64::MAX.ilog10() + 1) as usize;

pub fn part_one(input: &str) -> Option<u64> {
    let res = input_iter(input)
        .flatten()
        .map(|n| {
            let mut current_number = n;
            let mut digits: ArrayVec<u8, MAX_DIGITS> = ArrayVec::new();
            while current_number != 0 {
                digits.push((current_number % 10) as u8);
                current_number /= 10;
            }
            (n, digits)
        })
        .filter(|(_number, digits)| {
            let (lower, upper) = digits.split_at(digits.len() / 2);
            lower == upper
        })
        .map(|(number, _digits)| number)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input_iter(input)
        .flatten()
        .map(|n| {
            let mut current_number = n;
            let mut digits: ArrayVec<u8, MAX_DIGITS> = ArrayVec::new();
            while current_number != 0 {
                digits.push((current_number % 10) as u8);
                current_number /= 10;
            }
            (n, digits)
        })
        .filter(|(_number, digits)| digits.len() % 2 == 0)
        .filter(|(_number, digits)| {
            let mut repeated_buf: ArrayVec<u8, { (MAX_DIGITS - 1) * 2 }> = ArrayVec::new();
            repeated_buf.extend(digits.iter().skip(1).copied());
            repeated_buf.extend(digits.iter().take(digits.len() - 1).copied());
            memmem::find(&repeated_buf, digits).is_some()
        })
        .map(|(number, _digits)| number)
        .sum();
    Some(res)
}
