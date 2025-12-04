use ndarray::Array2;

advent_of_code::solution!(4);

fn input_iter(input: &str) -> impl Iterator<Item = impl ExactSizeIterator<Item = bool>> {
    input
        .split('\n')
        .map(|line| line.bytes().map(|b| b == b'@'))
}

pub fn part_one(input: &str) -> Option<usize> {
    let width = input_iter(input).next().unwrap().len();
    let height = input_iter(input).count();
    let mut array = Array2::from_elem((height, width), 0u8);
    for (row, line) in input_iter(input).enumerate() {
        for (column, toiler_paper) in line.enumerate() {
            array[[row, column]] |= (toiler_paper as u8) << 7;
            if !toiler_paper {
                continue;
            }
            if column != 0 {
                array[[row, column - 1]] += 1;
                if row != 0 {
                    array[[row - 1, column - 1]] += 1;
                }
                if row + 1 < height {
                    array[[row + 1, column - 1]] += 1;
                }
            }
            if row != 0 {
                array[[row - 1, column]] += 1;
            }
            if row + 1 < height {
                array[[row + 1, column]] += 1;
            }
            if column + 1 < width {
                array[[row, column + 1]] += 1;
                if row != 0 {
                    array[[row - 1, column + 1]] += 1;
                }
                if row + 1 < height {
                    array[[row + 1, column + 1]] += 1;
                }
            }
        }
    }
    Some(
        array
            .iter()
            .filter(|&&b| b >> 7 != 0)
            .map(|&c| c & !(1 << 7))
            .filter(|&c| c < 4)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let width = input_iter(input).next().unwrap().len();
    let height = input_iter(input).count();
    let mut array = Array2::from_elem((height, width), false);
    for (row, line) in input_iter(input).enumerate() {
        for (column, toiler_paper) in line.enumerate() {
            array[[row, column]] = toiler_paper;
        }
    }
    let mut swap = array.clone();
    let mut removed = 0;
    loop {
        let center_points = (0..height).flat_map(|y| (0..width).map(move |x| (y, x)));
        let mut removed_any = false;
        for (row, column) in center_points {
            if !array[[row, column]] {
                continue;
            }
            let mut neighbors = 0;
            let mut add_if_exists = |r, c| {
                if array.get((r, c)).is_some_and(|&toiler_paper| toiler_paper) {
                    neighbors += 1;
                }
            };
            add_if_exists(row.wrapping_sub(1), column.wrapping_sub(1));
            add_if_exists(row.wrapping_sub(1), column);
            add_if_exists(row.wrapping_sub(1), column + 1);
            add_if_exists(row, column.wrapping_sub(1));
            add_if_exists(row, column + 1);
            add_if_exists(row + 1, column.wrapping_sub(1));
            add_if_exists(row + 1, column);
            add_if_exists(row + 1, column + 1);
            if neighbors < 4 {
                swap[[row, column]] = false;
                removed += 1;
                removed_any = true;
            }
        }
        array
            .iter_mut()
            .zip(swap.iter())
            .for_each(|(out, swap)| *out = *swap);
        if !removed_any {
            break;
        }
    }
    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
