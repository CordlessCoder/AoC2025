use std::collections::{BTreeMap, BinaryHeap};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Pos {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Pos {
    pub fn distance_squared(&self, other: Self) -> u64 {
        let Self { x, y, z } = *self;
        let diff_x = other.x.abs_diff(x);
        let diff_y = other.y.abs_diff(y);
        let diff_z = other.z.abs_diff(z);
        diff_x * diff_x + diff_y * diff_y + diff_z * diff_z
    }
}

fn input_iter(input: &str) -> impl Iterator<Item = Pos> {
    input.lines().map(|line| {
        let mut elements = line.split(',').map(|n| n.parse().unwrap());
        let [x, y, z] = core::array::from_fn(|_| elements.next().unwrap());
        Pos { x, y, z }
    })
}

#[cfg(test)]
const CONNECTIONS: u32 = 10;
#[cfg(not(test))]
const CONNECTIONS: u32 = 1000;

pub fn part_one(input: &str) -> Option<u64> {
    let mut positions_and_group_indices: Vec<(usize, Pos)> =
        input_iter(input).enumerate().collect();
    let mut connections: Vec<(usize, usize)> = (1..positions_and_group_indices.len())
        .flat_map(|max| (0..max).map(move |min| (min, max)))
        .collect();
    for _ in 0..10 {
        let (idx, &(first, second)) = connections
            .iter()
            .enumerate()
            .min_by_key(|&(_, &(a, b))| {
                positions_and_group_indices[a]
                    .1
                    .distance_squared(positions_and_group_indices[b].1)
            })
            .unwrap();
        connections.swap_remove(idx);
        let [(first, a), (second, b)] = positions_and_group_indices
            .get_disjoint_mut([first, second])
            .unwrap();
        dbg!(a, b, &first, &second);
        if *first < *second {
            *second = *first;
        } else {
            *first = *second;
        }
    }
    let mut circuit_sizes: BTreeMap<usize, usize> = BTreeMap::new();
    for (circuit, _) in positions_and_group_indices {
        *circuit_sizes.entry(circuit).or_default() += 1;
    }
    dbg!(&circuit_sizes);
    let mut largest_sizes = BinaryHeap::from_iter(circuit_sizes.into_values());
    let mut product = 1;
    for _ in 0..3 {
        let size = largest_sizes.pop().unwrap();
        product *= size as u64;
    }
    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
