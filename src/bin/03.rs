advent_of_code::solution!(3);

type Bank = [u8; 100];

fn input_iter(input: &str) -> impl Iterator<Item = Bank> {
    input.lines().map(|line| {
        let bytes = line.trim().as_bytes();
        let as_bank: &Bank = bytes.try_into().unwrap();
        as_bank.map(|byte| byte.wrapping_sub(b'0'))
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input_iter(input)
        .map(|bank| {
            let (max_idx, &max_val) = bank.iter().enumerate().max_by_key(|&(_idx, v)| v).unwrap();
            let mut max = 0;
            if let Some((_, max_to_the_left)) = bank
                .iter()
                .enumerate()
                .take(max_idx)
                .max_by_key(|&(_idx, v)| v)
            {
                max = max_to_the_left * 10 + max_val;
            }
            if let Some((_, max_to_the_right)) = bank
                .iter()
                .enumerate()
                .skip(max_idx + 1)
                .max_by_key(|&(_idx, v)| v)
            {
                max = (max_val * 10 + max_to_the_right).max(max);
            }
            max as u64
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    const WIDTH: usize = 12;
    let res = input_iter(input)
        .map(|bank| {
            let mut start = 0;
            let mut digits_found = 0;
            let mut find_digit = || {
                let (best_digit_pos, value) = bank
                    .into_iter()
                    .enumerate()
                    .take(bank.len() + digits_found - (WIDTH - 1))
                    .skip(start)
                    .rev()
                    .max_by_key(|&(_idx, v)| v)
                    .unwrap();
                start = best_digit_pos + 1;
                digits_found += 1;
                value
            };
            (0..WIDTH).fold(0u64, |total, _| total * 10 + find_digit() as u64)
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
