advent_of_code::solution!(3);

type Bank = [u8; 100];

#[cfg(feature = "unsafe_optimizations")]
fn input_iter(input: &str) -> impl Iterator<Item = Bank> {
    let mut last = 0;
    let line_separated = memchr::memchr_iter(b'\n', input.as_bytes()).map(move |end| {
        let segment = unsafe { input.get_unchecked(last..end) };
        last = end + 1;
        segment
    });
    line_separated.map(|line| {
        let bytes = line.trim().as_bytes();
        // SAFETY: This is guaranteed by the input format
        let as_bank: &Bank = unsafe { bytes.try_into().unwrap_unchecked() };
        let mut bank = *as_bank;
        // Encourage vectorization with chunked subtraction
        let mut chunks = bank.chunks_exact_mut(32);
        chunks.by_ref().for_each(|c| {
            c.iter_mut().for_each(|c| *c -= b'0');
        });
        chunks.into_remainder().iter_mut().for_each(|c| *c -= b'0');
        bank
    })
}

#[cfg(not(feature = "unsafe_optimizations"))]
fn input_iter(input: &str) -> impl Iterator<Item = Bank> {
    input.lines().map(|line| {
        let bytes = line.trim().as_bytes();
        let as_bank: &Bank = bytes.try_into().unwrap();
        as_bank.map(|byte| byte.wrapping_sub(b'0'))
    })
}

#[inline(always)]
fn find_max_idx(data: &[u8]) -> Option<(usize, u8)> {
    const WIDTH: usize = 32;
    if data.is_empty() {
        return None;
    }
    let mut max = 0;
    let mut max_idx = 0;
    let mut chunks = data.chunks_exact(WIDTH);
    let remainder_offset = chunks.len() * WIDTH;
    for (chunk_idx, chunk) in chunks.by_ref().enumerate() {
        let offset = chunk_idx * WIDTH;
        chunk.iter().enumerate().for_each(|(idx, value)| {
            if *value > max {
                max_idx = offset.wrapping_add(idx);
                max = *value;
            }
        });
    }
    chunks
        .remainder()
        .iter()
        .enumerate()
        .for_each(|(idx, value)| {
            if *value > max {
                max_idx = remainder_offset.wrapping_add(idx);
                max = *value;
            }
        });
    Some((max_idx, max))
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input_iter(input)
        .map(|bank| {
            #[cfg(feature = "unsafe_optimizations")]
            let (max_idx, max_val) = unsafe { find_max_idx(&bank).unwrap_unchecked() };
            #[cfg(not(feature = "unsafe_optimizations"))]
            let (max_idx, max_val) = find_max_idx(&bank).unwrap();
            let mut max = 0;
            if let Some((_, max_to_the_left)) = find_max_idx(&bank[..max_idx]) {
                max = max_to_the_left * 10 + max_val;
            }
            if let Some((_, max_to_the_right)) = find_max_idx(&bank[max_idx + 1..]) {
                max = (max_val * 10 + max_to_the_right).max(max);
            }
            max as u64
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    const WIDTH: usize = 12;
    let res = input_iter(input)
        .map(|bank| {
            let mut start = 0;
            let mut digits_found = 0;
            let mut find_digit = || {
                debug_assert!(digits_found < WIDTH);
                #[cfg(not(feature = "unsafe_optimizations"))]
                let (best_digit_offset, value) =
                    find_max_idx(&bank[start..bank.len() + digits_found - (WIDTH - 1)]).unwrap();
                #[cfg(feature = "unsafe_optimizations")]
                let (best_digit_offset, value) = unsafe {
                    find_max_idx(bank.get_unchecked(start..bank.len() + digits_found - (WIDTH - 1)))
                        .unwrap_unchecked()
                };
                start += best_digit_offset + 1;
                digits_found += 1;
                value
            };
            (0..WIDTH).fold(0u64, |total, _| total * 10 + find_digit() as u64)
        })
        .sum();
    Some(res)
}
