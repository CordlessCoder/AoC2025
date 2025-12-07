advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Source,
    Splitter,
}

fn parse_input(input: &str) -> impl FnMut(&mut Vec<Cell>) {
    let mut lines = input.lines();
    move |out| {
        out.clear();
        let Some(line) = lines.next() else {
            return;
        };
        let cells = line.bytes().map(|b| match b {
            b'S' => Cell::Source,
            b'^' => Cell::Splitter,
            b'.' => Cell::Empty,
            c => panic!("Unexpected character: {}", c as char),
        });
        out.extend(cells);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut parse_line_into = parse_input(input);
    let mut buf = Vec::new();
    parse_line_into(&mut buf);
    let mut beams: Vec<bool> = buf.iter().map(|&b| b == Cell::Source).collect();
    let mut beams_swap = beams.clone();
    let mut splits = 0;
    while {
        parse_line_into(&mut buf);
        !buf.is_empty()
    } {
        for idx in 0..beams.len() {
            if !beams[idx] {
                continue;
            }
            if buf[idx] != Cell::Splitter {
                continue;
            }
            splits += 1;
            if let Some(left) = beams_swap.get_mut(idx.wrapping_sub(1)) {
                *left = true;
            }
            beams_swap[idx] = false;
            if let Some(right) = beams_swap.get_mut(idx.wrapping_add(1)) {
                *right = true;
            }
        }
        std::mem::swap(&mut beams, &mut beams_swap);
        beams_swap.clear();
        beams_swap.extend(&beams);
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut parse_line_into = parse_input(input);
    let mut buf = Vec::new();
    parse_line_into(&mut buf);
    // In how many timelines is there a particle in this cell
    let mut beam_timeline_counts: Vec<u64> = buf
        .iter()
        .map(|&b| if b == Cell::Source { 1 } else { 0 })
        .collect();
    let mut btc_swap = beam_timeline_counts.clone();
    while {
        parse_line_into(&mut buf);
        !buf.is_empty()
    } {
        for idx in 0..beam_timeline_counts.len() {
            let count = beam_timeline_counts[idx];
            if count == 0 {
                continue;
            }
            if buf[idx] != Cell::Splitter {
                continue;
            }
            if let Some(left) = btc_swap.get_mut(idx.wrapping_sub(1)) {
                *left += count;
            }
            btc_swap[idx] = 0;
            if let Some(right) = btc_swap.get_mut(idx.wrapping_add(1)) {
                *right += count;
            }
        }
        std::mem::swap(&mut beam_timeline_counts, &mut btc_swap);
        btc_swap.clear();
        btc_swap.extend(&beam_timeline_counts);
    }
    Some(beam_timeline_counts.iter().copied().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
