use bitvec::prelude::*;
use itertools::Itertools;
use quadtree_rs::{
    area::{Area, AreaBuilder},
    point::Point,
};

advent_of_code::solution!(9);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn area_to(self, other: Self) -> u64 {
        let Self { x, y } = self;
        (other.x.abs_diff(x) + 1) as u64 * (other.y.abs_diff(y) + 1) as u64
    }
    pub fn as_point(self) -> Point<u32> {
        let Self { x, y } = self;
        Point { x, y }
    }
    pub fn into_area(self, to: Self) -> Area<u32> {
        let from = self;
        let top_left = Point {
            x: from.x.min(to.x),
            y: from.y.min(to.y),
        };
        let width = from.x.max(to.x) + 1 - top_left.x;
        let height = from.y.max(to.y) + 1 - top_left.y;
        AreaBuilder::default()
            .anchor(top_left)
            .dimensions((width, height))
            .build()
            .unwrap()
    }
}

fn input_iter(input: &str) -> impl Iterator<Item = Pos> {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let [x, y] = [x, y].map(|coordinate| coordinate.parse().unwrap());
        Pos { x, y }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let positions: Vec<Pos> = input_iter(input).collect();
    let area = positions
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| a.area_to(b))
        .max()
        .unwrap();
    Some(area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let positions: Vec<Pos> = input_iter(input).collect();
    // // positions.sort_unstable();
    let max_x = positions.iter().map(|pos| pos.x).max().unwrap() as usize;
    let max_y = positions.iter().map(|pos| pos.y).max().unwrap() as usize;
    let mut buf = bitvec!(0; max_x * max_y);
    // let mut heap = BinaryHeap::new_by_key(|(a, b): &(Pos, Pos)| a.area_to(*b));
    // heap.extend(positions.iter().copied().tuple_combinations::<(Pos, Pos)>());
    // for (a, b) in heap.into_iter_sorted() {
    //     let area = a.into_area(b);
    //     dbg!(area, quadtree.query(area).collect_vec());
    // }
    // // Some(area)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
