///! Day 2 - The Gift Shop
use std::io::BufRead;

fn main() -> anyhow::Result<()> {
    let input = advent_code_25::open_input_file()?;
    let answer = solve_for(input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn solve_for<R: BufRead>(input: R) -> anyhow::Result<usize> {
    let mut nb_invalid_id = 0;
    for range in ranges_from_input(input) {
        nb_invalid_id += sum_invalid_ids_in_range(range?);
    }
    Ok(nb_invalid_id)
}

fn ranges_from_input<R: BufRead>(input: R) -> impl Iterator<Item = anyhow::Result<Range>> {
    RangeIter
}

/// Closed range spanning from .0 to .1 inclusive.
#[derive(Debug, Copy, Clone)]
struct Range(usize, usize);

struct RangeIter;

impl Iterator for RangeIter {
    type Item = anyhow::Result<Range>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn sum_invalid_ids_in_range(range: Range) -> usize {
    let mut nb_invalid_id = 0;
    for id in range.0..=range.1 {
        if is_invalid_id(id) {
            nb_invalid_id += 1
        }
    }
    nb_invalid_id
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
