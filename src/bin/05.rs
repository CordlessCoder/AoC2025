use rangemap::RangeInclusiveSet;

advent_of_code::solution!(5);

#[cfg(not(feature = "unsafe_optimizations"))]
fn parse_input(input: &str) -> (RangeInclusiveSet<u64>, impl Iterator<Item = u64>) {
    let mut lines = input.lines();
    let mut ranges = RangeInclusiveSet::new();
    lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let [start, end] = [start, end].map(|n| n.parse().unwrap());
            (start, end)
        })
        .for_each(|(start, end)| ranges.insert(start..=end));
    (ranges, lines.map(|line| line.parse().unwrap()))
}
#[cfg(feature = "unsafe_optimizations")]
fn parse_input(input: &str) -> (RangeInclusiveSet<u64>, impl Iterator<Item = u64>) {
    use advent_of_code::split::split_by;

    let mut lines = split_by(input, '\n');
    let ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|range| {
            // SAFETY: This is guaranteed by the input format
            let (start, end) = unsafe { range.split_once('-').unwrap_unchecked() };
            // SAFETY: This is guaranteed by the input format
            let [start, end] = [start, end].map(|n| unsafe { n.parse().unwrap_unchecked() });
            (start, end)
        });
    let ranges = ranges.fold(RangeInclusiveSet::new(), |mut set, (start, end)| {
        set.insert(start..=end);
        set
    });
    (ranges, lines.map(|line| line.parse().unwrap()))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, input) = parse_input(input);
    Some(input.filter(|id| ranges.contains(id)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, _) = parse_input(input);
    Some(ranges.iter().map(|r| r.clone().count()).sum())
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
        assert_eq!(result, Some(14));
    }
}
