// Day 3 - Lobby

use std::io::BufRead;

mod iterator;
use iterator::IteratorExt;

fn main() -> anyhow::Result<()> {
    let input = advent_code_25::open_input_file()?;
    let answer = solve_for(input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn solve_for<R: BufRead>(input: R) -> anyhow::Result<usize> {
    let mut max_joltage = 0;

    // XXX: Does not work with Windows text files.
    for bank in input.split(b'\n') {
        let bank = bank?;
        if is_bank_valid(&bank) {
            let left_max_index = (&bank[0..bank.len() - 1])
                .iter()
                .first_max_position()
                .unwrap();
            let right_max_index = left_max_index
                + 1
                + (&bank[left_max_index + 1..])
                    .iter()
                    .first_max_position()
                    .unwrap();
            max_joltage += bank_joltage(&bank, left_max_index, right_max_index);
        } else {
            anyhow::bail!("ill-formed bank: {:?}", bank);
        }
    }

    Ok(max_joltage)
}

/// Checks that `bank` contains only digits and is long enough.
fn is_bank_valid(bank: &[u8]) -> bool {
    bank.len() >= 2 && bank.iter().all(|b| b'0' <= *b && *b <= b'9')
}

/// Returns joltage for specified batteries.
fn bank_joltage(bank: &[u8], left_index: usize, right_index: usize) -> usize {
    from_digit(bank[left_index]) * 10 + from_digit(bank[right_index])
}

fn from_digit(b: u8) -> usize {
    debug_assert!(b'0' <= b && b <= b'9');
    (b - b'0') as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bank_valid() {
        // too small
        assert!(!is_bank_valid(b""));
        assert!(!is_bank_valid(b"1"));

        assert!(is_bank_valid(b"12"));
        assert!(is_bank_valid(b"123"));

        // non-digits
        assert!(!is_bank_valid(b"1a"));
    }

    #[test]
    fn test_bank_joltage() {
        assert_eq!(bank_joltage(b"12", 0, 1), 12);
        assert_eq!(bank_joltage(b"96", 0, 1), 96);
    }
}
