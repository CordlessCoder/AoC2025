advent_of_code::solution!(6);

enum Operation {
    Mul,
    Add,
}

fn parse_input(
    input: &str,
) -> (
    Vec<Operation>,
    impl Iterator<Item = impl Iterator<Item = u64>>,
) {
    let mut lines = input.lines();
    let ops = lines
        .next_back()
        .unwrap_or_default()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|op| match op {
            "*" => Operation::Mul,
            "+" => Operation::Add,
            _ => unreachable!(),
        })
        .collect();
    let values = lines.map(|line| {
        line.split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
    });
    (ops, values)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ops, mut values) = parse_input(input);
    let mut reduce: Vec<(u64, Operation)> = values
        .next()
        .map(|values| values.zip(ops).collect())
        .unwrap_or_default();
    for line in values {
        for ((reducer, operation), value) in reduce.iter_mut().zip(line) {
            *reducer = match operation {
                Operation::Add => *reducer + value,
                Operation::Mul => *reducer * value,
            }
        }
    }
    Some(reduce.iter().map(|(v, _)| v).sum())
}

fn parse_col<'a>(rows: impl Iterator<Item = &'a str>, column: usize) -> Option<u64> {
    let mut accumulator = None;
    for row in rows {
        let &col = row.as_bytes().get(column)?;
        if col == b' ' {
            continue;
        }
        let val = (col - b'0') as u64;
        accumulator = Some(accumulator.unwrap_or_default() * 10 + val);
    }
    accumulator
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let ops = lines
        .next_back()
        .unwrap_or_default()
        .bytes()
        .enumerate()
        .filter(|(_, b)| *b != b' ')
        .map(|(idx, op)| {
            (
                idx,
                match op {
                    b'*' => Operation::Mul,
                    b'+' => Operation::Add,
                    _ => unreachable!(),
                },
            )
        });
    let lines: Vec<&str> = lines.collect();
    let mut sum = 0;
    for (skip, op) in ops {
        let rows = lines.iter().map(|line| &line[skip..]);
        let mut accumulator = parse_col(rows.clone(), 0).unwrap();
        for column in 1.. {
            let Some(val) = parse_col(rows.clone(), column) else {
                break;
            };
            accumulator = match op {
                Operation::Mul => accumulator * val,
                Operation::Add => accumulator + val,
            }
        }
        sum += accumulator;
    }
    Some(sum)
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
