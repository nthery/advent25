///! Day 2 - The Gift Shop
use std::io::{BufRead, Split};

fn main() -> anyhow::Result<()> {
    let input = advent_code_25::open_input_file()?;
    let answer = solve_for(input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn solve_for<R: BufRead>(input: R) -> anyhow::Result<usize> {
    let mut acc = 0;
    for range in ranges_from_input(input)? {
        acc += sum_invalid_ids_in_range(range?);
    }
    Ok(acc)
}

/// Returns iterator that parses `input` into a sequence of ID ranges.
fn ranges_from_input<R: BufRead>(
    input: R,
) -> anyhow::Result<impl Iterator<Item = anyhow::Result<Range>>> {
    Ok(RangeIter::new(input.split(b',')))
}

/// Closed range spanning from .0 to .1 inclusive.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range(usize, usize);

/// Iterator that yields Range values.
struct RangeIter<R: BufRead> {
    ranges: Split<R>,
}

impl<R: BufRead> RangeIter<R> {
    fn new(ranges: Split<R>) -> Self {
        Self { ranges }
    }
}

impl<R: BufRead> Iterator for RangeIter<R> {
    type Item = anyhow::Result<Range>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ranges.next() {
            Some(range) => match range {
                Ok(range) => Some(parse_id_range(&range)),
                Err(e) => Some(Err(anyhow::anyhow!("failed to parse ID range: {}", e))),
            },
            None => None,
        }
    }
}

fn parse_id_range(input: &[u8]) -> anyhow::Result<Range> {
    if let Some(pos) = input.iter().position(|&c| c == b'-') {
        Ok(Range(
            parse_id(&input[..pos])?,
            parse_id(&input[pos + 1..])?,
        ))
    } else {
        anyhow::bail!("missing '-' in ID range: {:?}", input);
    }
}

fn parse_id(input: &[u8]) -> anyhow::Result<usize> {
    if input.is_empty() {
        anyhow::bail!("empty ID");
    }
    let mut n = 0_usize;
    for &b in input {
        if !b.is_ascii_digit() {
            anyhow::bail!("unexpected character in ID: {}", b)
        }
        n = n * 10 + (b - b'0') as usize;
    }
    Ok(n)
}

fn sum_invalid_ids_in_range(range: Range) -> usize {
    let mut acc = 0;
    for id in range.0..=range.1 {
        if is_invalid_id(id) {
            acc += id
        }
    }
    acc
}

fn is_invalid_id(id: usize) -> bool {
    let nb_digits = nb_digits(id);
    if nb_digits == 1 || (nb_digits & 1) != 0 {
        false
    } else {
        let power = 10_usize.pow(nb_digits / 2);
        let left = id / power;
        let right = id % power;
        left == right
    }
}

/// Returns number of base-10 digits in `n`.
fn nb_digits(n: usize) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_id() {
        assert!(matches!(parse_id(b"1"), Ok(1)));
        assert!(matches!(parse_id(b"123"), Ok(123)));
        assert!(matches!(parse_id(b"123!"), Err(_)));
    }

    #[test]
    fn test_parse_id_range() {
        assert!(matches!(parse_id_range(b"1-2"), Ok(Range(1, 2))));
        assert!(matches!(parse_id_range(b"12-99"), Ok(Range(12, 99))));
        assert!(matches!(parse_id_range(b"12"), Err(_)));
        assert!(matches!(parse_id_range(b"12-"), Err(_)));
        assert!(matches!(parse_id_range(b"-12"), Err(_)));
    }

    #[test]
    fn test_parse_ranges() -> anyhow::Result<()> {
        assert_eq!(parse_input(b"")?, vec![]);
        assert_eq!(parse_input(b"1-2")?, vec![Range(1, 2)]);
        assert_eq!(parse_input(b"1-2,3-4")?, vec![Range(1, 2), Range(3, 4)]);
        Ok(())
    }

    fn parse_input(input: &[u8]) -> anyhow::Result<Vec<Range>> {
        let reader = Cursor::new(input);
        ranges_from_input(reader)?.collect::<anyhow::Result<Vec<Range>>>()
    }

    #[test]
    fn digits() {
        assert_eq!(nb_digits(0), 1);
        assert_eq!(nb_digits(9), 1);
        assert_eq!(nb_digits(10), 2);
        assert_eq!(nb_digits(99), 2);
        assert_eq!(nb_digits(100), 3);
        assert_eq!(nb_digits(999), 3);
    }

    #[test]
    fn id_with_odd_number_of_digits_is_valid() {
        assert!(!is_invalid_id(1));
    }

    #[test]
    fn id_without_repeating_sequence_is_valid() {
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(1234));
    }

    #[test]
    fn id_with_repeating_sequence_is_invalid() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(1212));
        assert!(is_invalid_id(123123));
    }
}
