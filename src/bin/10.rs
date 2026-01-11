use bitvec::bitarr;
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
        let _joltages = segments.next_back().unwrap();
        let button_iter = segments.map(|segment| {
            segment[1..segment.len() - 1]
                .split(',')
                .map(|light| light.parse().unwrap())
                .fold(LightBits::default(), |bits, light: u8| {
                    bits | (1u16 << light)
                })
        });
        Machine {
            target,
            button_iter,
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut buttons = Vec::new();
    Some(
        input_iter(input)
            .map(
                |Machine {
                     target,
                     button_iter,
                 }| {
                    let mut visited_states = bitarr!(0; (LightBits::MAX as usize + 1));
                    visited_states.set(0, true);
                    buttons.clear();
                    buttons.extend(button_iter);
                    if target == 0 {
                        return 0;
                    }
                    for presses in 1..=buttons.len() {
                        let mut new_states = visited_states;
                        visited_states
                            .iter_ones()
                            .map(|state| state as LightBits)
                            .for_each(|lights| {
                                for &button in &buttons {
                                    let new_state = lights ^ button;
                                    new_states.set(new_state as usize, true);
                                }
                            });
                        visited_states = new_states;
                        if *visited_states.get(target as usize).unwrap() {
                            return presses as u64;
                        }
                    }
                    0
                },
            )
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
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
