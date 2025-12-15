use itertools::Itertools;

advent_of_code::solution!(10);

type LightBits = u16;

struct Machine<B: Iterator<Item = LightBits>> {
    target: LightBits,
    button_iter: B,
}

fn input_iter(input: &str) -> impl Iterator<Item = Machine<impl Iterator<Item = LightBits>>> {
    input.lines().map(|line| {
        let mut segments = line.split(' ');
        let target_lights = segments.next().unwrap();
        let target = target_lights.as_bytes()[1..target_lights.len() - 1]
            .iter()
            .map(|&indicator| indicator == b'#')
            .enumerate()
            .fold(LightBits::default(), |bits, (light, enabled)| {
                bits | ((enabled as LightBits) << light)
            });
        let button_iter = segments.map(|segment| {
            segment[1..segment.len() - 1]
                .split(',')
                .map(|light| light.parse().unwrap())
                .fold(LightBits::default(), |bits, light: u8| bits | (1 << light))
        });
        Machine { target, button_iter}
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut buttons = Vec::new();
    Some(
        input_iter(input)
            .map(|Machine { target, button_iter }| {
                buttons.clear();
                buttons.extend(button_iter);
                for presses in 1..buttons.len() {
                }
                0
            })
            .sum(),
    )
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
