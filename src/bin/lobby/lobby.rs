// Day 3 - Lobby

use std::io::BufRead;

mod iterator;
use iterator::IteratorExt;

// Number of batteries to turn on per bank.
const NB_BATTERIES_ON: usize = 12;

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
            max_joltage += max_joltage_for_bank(&bank);
        } else {
            anyhow::bail!("ill-formed bank: {:?}", bank);
        }
    }

    Ok(max_joltage)
}

fn max_joltage_for_bank(bank: &[u8]) -> usize {
    let mut joltage = 0;

    // Indices of window in bank where to search for next battery to turn on.
    let mut i_first = 0;
    let mut i_last = bank.len() - NB_BATTERIES_ON;

    for _ in 0..NB_BATTERIES_ON {
        let i = (bank[i_first..=i_last])
            .iter()
            .first_max_position()
            .unwrap();
        joltage = joltage * 10 + from_digit(bank[i_first + i]);
        i_first += i + 1;
        i_last += 1;
    }

    joltage
}

/// Checks that `bank` contains only digits and is long enough.
fn is_bank_valid(bank: &[u8]) -> bool {
    bank.len() >= NB_BATTERIES_ON && bank.iter().all(|b| b'0' <= *b && *b <= b'9')
}

fn from_digit(b: u8) -> usize {
    debug_assert!(b.is_ascii_digit());
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
        assert!(!is_bank_valid(b"12345678901"));

        assert!(is_bank_valid(b"123456789012"));
        assert!(is_bank_valid(b"1234567890123"));

        // non-digits
        assert!(!is_bank_valid(b"123456789012@"));
    }

    #[test]
    fn test_max_joltage_for_bank() {
        assert_eq!(max_joltage_for_bank(b"987654321111111"), 987654321111);
        assert_eq!(max_joltage_for_bank(b"811111111111119"), 811111111119);
        assert_eq!(max_joltage_for_bank(b"234234234234278"), 434234234278);
        assert_eq!(max_joltage_for_bank(b"818181911112111"), 888911112111);
    }
}
