advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|line| {
            let (prefix, distance) = line.split_at(1);
            let offset: i32 = distance
                .parse()
                .expect("Failed to parse input distance");
            if prefix == "R" {
                offset
            } else {
                -offset
            }
        });
    let res = input
        .scan(50, |pointing_at, offset| {
            *pointing_at = (*pointing_at + offset) % 100;
            Some(*pointing_at)
        })
        .filter(|&p| p == 0)
        .count();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| {
            let (prefix, distance) = line.split_at(1);
            let offset: i32 = distance
                .parse()
                .expect("Failed to parse input distance");
            if prefix == "R" {
                offset
            } else {
                -offset
            }
        });
    let res = input
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
