use std::cmp::Ordering;

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
    let input = input_iter(input);
    let mut count = 0;
    let mut position = 50;

    input.for_each(|offset| {
        let raw_value = position + offset;
        position = raw_value.wrapping_rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    });
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut position = 50;
    let mut count = 0;

    let input = input_iter(input);

    for offset in input {
        let raw_value = position + offset;
        position = raw_value.wrapping_rem_euclid(100);

        match raw_value.cmp(&0) {
            Ordering::Less => {
                count += (raw_value / 100).unsigned_abs();
                if raw_value != offset {
                    count += 1;
                }
            }
            Ordering::Equal if raw_value != offset => count += 1,
            Ordering::Greater => count += raw_value.unsigned_abs() / 100,
            _ => (),
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
