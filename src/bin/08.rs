use std::{cell::Cell, collections::BTreeMap, rc::Rc};

use binary_heap_plus::BinaryHeap;

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
        let x = other.x.abs_diff(x);
        let y = other.y.abs_diff(y);
        let z = other.z.abs_diff(z);
        x * x + y * y + z * z
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
const CONNECTIONS: usize = 10;
#[cfg(not(test))]
const CONNECTIONS: usize = 1000;

pub fn part_one(input: &str) -> Option<u64> {
    let mut positions_and_group_indices: Vec<(Rc<Cell<usize>>, Pos)> = input_iter(input)
        .enumerate()
        .map(|(group, pos)| (Rc::new(Cell::new(group)), pos))
        .collect();
    let mut connections: Vec<(usize, usize)> = (1..positions_and_group_indices.len())
        .flat_map(|max| (0..max).map(move |min| (min, max)))
        .collect();
    let sort_key = |&(a, b): &(usize, usize)| {
        positions_and_group_indices[a]
            .1
            .distance_squared(positions_and_group_indices[b].1)
    };
    connections.select_nth_unstable_by_key(CONNECTIONS, sort_key);
    connections.truncate(CONNECTIONS);
    connections.sort_unstable_by_key(sort_key);
    for (first_idx, second_idx) in connections.into_iter() {
        let [(first, _), (second, _)] = positions_and_group_indices
            .get_disjoint_mut([first_idx, second_idx])
            .unwrap();
        let (min, max) = if first.get() < second.get() {
            (first, second)
        } else {
            (second, first)
        };
        let max_val = max.get();
        let min_val = min.get();
        max.set(min_val);
        *max = min.clone();
        if let Ok([(first, _), (second, _), (old_group, _)]) =
            positions_and_group_indices.get_disjoint_mut([first_idx, second_idx, max_val])
        {
            let min = (old_group).min(second).min(first).clone();
            let min_val = min.get();
            old_group.set(min_val);
            *old_group = min.clone();
            first.set(min_val);
            *first = min.clone();
            second.set(min_val);
            *second = min;
        };
    }
    let mut circuit_sizes: BTreeMap<usize, usize> = BTreeMap::new();
    for ref_idx in 0..positions_and_group_indices.len() {
        let ref_pos_and_group = positions_and_group_indices[ref_idx].clone();
        let broadcasted_pos = positions_and_group_indices[ref_pos_and_group.0.get()].clone();
        positions_and_group_indices[ref_idx]
            .0
            .set(broadcasted_pos.0.get());
        positions_and_group_indices[ref_idx].0 = broadcasted_pos.0.clone();
        *circuit_sizes.entry(broadcasted_pos.0.get()).or_default() += 1;
    }
    let mut largest_sizes = std::collections::BinaryHeap::from_iter(circuit_sizes.into_values());
    let mut product = 1;
    for _ in 0..3 {
        let size = largest_sizes.pop().unwrap();
        product *= size as u64;
    }
    Some(product)
}

struct UnionFind {
    parents: Vec<usize>,
    sets: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sets: size,
        }
    }
    pub fn find(&self, element: usize) -> usize {
        if self.parents[element] == element {
            return element;
        }
        self.find(self.parents[element])
    }
    pub fn join(&mut self, a: usize, b: usize) {
        let a_rep = self.find(a);
        let b_rep = self.find(b);
        if a_rep != b_rep {
            self.sets -= 1;
        }
        self.parents[a_rep] = b_rep;
    }
    pub fn sets(&self) -> usize {
        self.sets
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let positions_and_group_indices: Vec<Pos> = input_iter(input).collect();
    let sort_key = |&(a, b): &(usize, usize)| {
        u64::MAX - positions_and_group_indices[a].distance_squared(positions_and_group_indices[b])
    };
    let mut connections = BinaryHeap::new_by_key(sort_key);
    for connection in
        (1..positions_and_group_indices.len()).flat_map(|max| (0..max).map(move |min| (min, max)))
    {
        connections.push(connection);
    }
    let mut sets = UnionFind::new(positions_and_group_indices.len());
    let mut last_connection = (0, 0);
    for (first_idx, second_idx) in connections.into_iter_sorted() {
        sets.join(first_idx, second_idx);
        if sets.sets() == 1 {
            last_connection = (first_idx, second_idx);
            break;
        }
    }
    let a = positions_and_group_indices[last_connection.0];
    let b = positions_and_group_indices[last_connection.1];
    Some(a.x * b.x)
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
        assert_eq!(result, Some(25272));
    }
}
