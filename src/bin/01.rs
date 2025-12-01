advent_of_code::solution!(1);

type Int = i32;

#[cfg(feature = "unsafe_optimizations")]
fn input_iter(input: &str) -> impl Iterator<Item = Int> {
    use std::hint::unreachable_unchecked;

    input.lines().map(str::as_bytes).map(|line| {
        let Some((direction, distance)) = line.split_first() else {
            // SAFETY: This is guaranteed by the input format
            unsafe { unreachable_unchecked() }
        };
        let distance = unsafe {
            match *distance {
                [hundreds, tens, ones] => (hundreds.unchecked_sub(b'0') as Int)
                    .unchecked_mul(100)
                    .unchecked_add(
                        (tens.unchecked_sub(b'0') as Int)
                            .unchecked_mul(10)
                            .unchecked_add(ones.unchecked_sub(b'0') as Int),
                    ),
                [tens, ones] => (tens.unchecked_sub(b'0') as Int)
                    .unchecked_mul(10)
                    .unchecked_add(ones.unchecked_sub(b'0') as Int),
                [ones] => ones.unchecked_sub(b'0') as Int,
                _ => {
                    // SAFETY: This is guaranteed by the input format
                    unreachable_unchecked()
                }
            }
        };
        if *direction == b'R' {
            distance
        } else {
            unsafe { distance.checked_neg().unwrap_unchecked() }
        }
    })
}
#[cfg(not(feature = "unsafe_optimizations"))]
fn input_iter(input: &str) -> impl Iterator<Item = Int> {
    input.lines().map(|line| {
        let (prefix, distance) = line.split_at(1);
        let offset: Int = distance.parse().expect("Failed to parse input distance");
        if prefix == "R" { offset } else { -offset }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input_iter(input)
        .scan(50, |pointing_at, offset| {
            *pointing_at = (*pointing_at + offset) % 100;
            Some(*pointing_at)
        })
        .map(|p| (p == 0) as u32)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input_iter(input)
        .scan(50, |pointing_at, offset| {
            let mut full_rotations = (offset / 100).unsigned_abs();
            let old_pos = *pointing_at;
            let new_pos = (old_pos + offset).rem_euclid(100);
            *pointing_at = new_pos;
            let went_to_zero = new_pos == 0 && old_pos != 0;
            let went_below_zero = offset.is_negative() && old_pos != 0 && new_pos > old_pos;
            let went_above_max = offset.is_positive() && new_pos < old_pos;
            if went_below_zero || went_to_zero || went_above_max {
                full_rotations += 1;
            }
            Some(full_rotations)
        })
        .sum();
    Some(res)
}
