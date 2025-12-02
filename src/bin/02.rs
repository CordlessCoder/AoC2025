use std::collections::BTreeSet;

advent_of_code::solution!(2);

#[cfg(not(feature = "unsafe_optimizations"))]
fn input_iter(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.trim().split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        let [start, end] = [start, end].map(|n| n.parse().unwrap());
        (start, end)
    })
}
#[cfg(feature = "unsafe_optimizations")]
fn input_iter(input: &str) -> impl Iterator<Item = (u64, u64)> {
    let mut last = 0;
    let comma_separated = memchr::memchr_iter(b',', input.as_bytes()).map(move |end| {
        let segment = unsafe { input.get_unchecked(last..end) };
        last = end + 1;
        segment
    });
    comma_separated.map(|range| {
        // SAFETY: This is guaranteed by the input format
        let (start, end) = unsafe { range.split_once('-').unwrap_unchecked() };
        // SAFETY: This is guaranteed by the input format
        let [start, end] = [start, end].map(|n| unsafe { n.parse().unwrap_unchecked() });
        (start, end)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input_iter(input)
        .flat_map(|(start, end)| {
            let min_exp = start.ilog10();
            let max_exp = end.ilog10();
            // Only keep odd exponents as those have an even number of digits, we can't get an odd
            // number of digits by concatenating a number to itself.
            let odd_exponents = (min_exp | 1..=max_exp).step_by(2);
            odd_exponents.map(move |exponent| -> u64 {
                let factor = 10u64.pow(exponent / 2 + 1) + 1;
                let min_sequence = start.div_ceil(factor).max(10u64.pow(exponent / 2));
                let max_sequence = (end / factor).min(10u64.pow(exponent / 2 + 1) - 1);
                (min_sequence..=max_sequence)
                    .map(|seq| seq * factor)
                    .filter(|candidate| (start..=end).contains(candidate))
                    .sum()
            })
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let invalid_iter = input_iter(input).flat_map(|(start, end)| {
        let min_exp = start.ilog10();
        let max_exp = end.ilog10();
        (min_exp..=max_exp).flat_map(move |exponent| {
            (2..=exponent + 1)
                // The number of digits (exponent + 1) must cleanly divide by repetitions
                .filter(move |repetitions| (exponent + 1) % repetitions == 0)
                .flat_map(move |repetitions| {
                    let digits_per_repetition = (exponent + 1) / repetitions;
                    let repeat_factor = 10u64.pow(digits_per_repetition);
                    let mut factor = 1;
                    for _ in 1..repetitions {
                        factor = factor * repeat_factor + 1;
                    }
                    let min_sequence = start
                        .div_ceil(factor)
                        .max(10u64.pow(exponent / repetitions));
                    let max_sequence =
                        (end / factor).min(10u64.pow((exponent + 1) / repetitions) - 1);
                    (min_sequence..=max_sequence)
                        .map(move |seq| seq * factor)
                        .filter(move |candidate| (start..=end).contains(candidate))
                })
        })
    });
    Some(invalid_iter.collect::<BTreeSet<u64>>().into_iter().sum())
}
