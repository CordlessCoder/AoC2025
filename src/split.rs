#[cfg(feature = "unsafe_optimizations")]
pub fn split_by(input: &str, pattern: char) -> impl Iterator<Item = &str> {
    let mut last = 0;
    assert!(pattern.is_ascii());
    let byte = pattern as u8;
    memchr::memchr_iter(byte, input.as_bytes())
        .chain(match input.bytes().last() {
            Some(b) if b == byte => core::iter::repeat_n(0, 0),
            Some(_) => core::iter::repeat_n(input.len(), 1),
            None => core::iter::repeat_n(0, 0),
        })
        .map(move |end| {
            let segment = unsafe { input.get_unchecked(last..end) };
            last = end + 1;
            segment
        })
}
#[cfg(not(feature = "unsafe_optimizations"))]
pub fn split_by(input: &str, pattern: char) -> impl Iterator<Item = &str> {
    assert!(pattern.is_ascii());
    input.split(pattern)
}
