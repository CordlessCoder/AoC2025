use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

advent_of_code::solution!(11);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device(pub [u8; 3]);

const OUT: Device = Device(*b"out");
const YOU: Device = Device(*b"you");
const SVR: Device = Device(*b"svr");
const FFT: Device = Device(*b"fft");
const DAC: Device = Device(*b"dac");

pub fn parse_input(input: &str) -> HashMap<Device, Vec<Device>> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let device = Device(line[..3].try_into().unwrap());
            (
                device,
                line[5..]
                    .split(|&b| b == b' ')
                    .map(|dev| Device(dev.try_into().unwrap())),
            )
        })
        .fold(HashMap::new(), |mut map, (input, outputs)| {
            map.insert(input, outputs.collect());
            map
        })
}

#[derive(Debug, Clone, Copy, Default)]
struct PathCounts {
    any: usize,
    dac: usize,
    fft: usize,
    complete: usize,
}

impl PathCounts {
    pub fn apply(self, node: Device) -> Self {
        let Self {
            any,
            dac,
            fft,
            complete,
        } = self;
        match node {
            FFT => Self {
                any: 0,
                dac: 0,
                fft: any,
                complete: dac + complete,
            },
            DAC => Self {
                any: 0,
                fft: 0,
                dac: any,
                complete: fft + complete,
            },
            _ => self,
        }
    }
}

impl Add for PathCounts {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Self {
            any,
            dac,
            fft,
            complete,
        } = self;
        Self {
            any: any + rhs.any,
            dac: dac + rhs.dac,
            fft: fft + rhs.fft,
            complete: complete + rhs.complete,
        }
    }
}

impl AddAssign for PathCounts {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = parse_input(input);
    let mut count_cache: HashMap<Device, usize> = HashMap::from_iter([(OUT, 1)]);
    fn count_paths_to_out(
        data: &HashMap<Device, Vec<Device>>,
        count_cache: &mut HashMap<Device, usize>,
        starting_device: Device,
    ) -> usize {
        if let Some(&count) = count_cache.get(&starting_device) {
            return count;
        }
        let mut paths = 0;
        for &output in data.get(&starting_device).unwrap() {
            if let Some(&count) = count_cache.get(&output) {
                paths += count;
                continue;
            }
            paths += count_paths_to_out(data, count_cache, output);
        }
        count_cache.insert(starting_device, paths);
        paths
    }
    Some(count_paths_to_out(&data, &mut count_cache, YOU))
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = parse_input(input);
    let mut count_cache: HashMap<Device, PathCounts> = HashMap::from_iter([(
        OUT,
        PathCounts {
            any: 1,
            dac: 0,
            fft: 0,
            complete: 0,
        },
    )]);
    fn count_paths_to_out(
        data: &HashMap<Device, Vec<Device>>,
        count_cache: &mut HashMap<Device, PathCounts>,
        starting_device: Device,
    ) -> PathCounts {
        if let Some(&count) = count_cache.get(&starting_device) {
            return count;
        }
        let mut paths = PathCounts::default();
        for &output in data.get(&starting_device).unwrap() {
            if let Some(&count) = count_cache.get(&output) {
                paths += count;
                continue;
            }
            paths += count_paths_to_out(data, count_cache, output);
        }
        paths = paths.apply(starting_device);
        count_cache.insert(starting_device, paths);
        paths
    }
    Some(count_paths_to_out(&data, &mut count_cache, SVR).complete)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
