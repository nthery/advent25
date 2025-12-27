mod dial;

use dial::Dial;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let path = args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("missing input file"))?;
    let input = BufReader::new(
        File::open(&path).map_err(|e| anyhow::anyhow!("failed to open {}: {}", path, e))?,
    );
    let answer = solve_for(input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn solve_for<R: BufRead>(mut input: R) -> anyhow::Result<usize> {
    let instructions = read_instructions(&mut input)?;
    Ok(execute_instructions(Dial::new(50), &instructions))
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    steps: usize,
}

fn read_instructions<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Rotation>> {
    let mut instructions = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line_number = i + 1;
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut chars = line.chars();
        let direction_char = chars.next().unwrap();
        let direction = match direction_char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => anyhow::bail!(
                "line {}: invalid direction: {}",
                line_number,
                direction_char
            ),
        };
        let steps_str = &line[1..];
        let steps: usize = steps_str.parse().map_err(|e| {
            anyhow::anyhow!("line {}: invalid step: {}: {}", line_number, steps_str, e)
        })?;
        instructions.push(Rotation { direction, steps });
    }
    Ok(instructions)
}

/// Turns `dial` as specified in `instructions` and returns number of times it
/// pointed to zero while being turned.
fn execute_instructions(mut dial: Dial, instructions: &[Rotation]) -> usize {
    let mut number_of_zeroes = if dial.position() == 0 { 1 } else { 0 };
    for rotation in instructions {
        number_of_zeroes += match rotation.direction {
            Direction::Right => dial.turn_right(rotation.steps),
            Direction::Left => dial.turn_left(rotation.steps),
        };
    }
    number_of_zeroes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_instructions_empty() {
        let data = "";
        let mut reader = Cursor::new(data.as_bytes());
        let v = read_instructions(&mut reader).unwrap();
        assert!(v.is_empty());
    }

    #[test]
    fn read_instructions_single() {
        let data = "R10\n";
        let mut reader = Cursor::new(data.as_bytes());
        let v = read_instructions(&mut reader).unwrap();
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0].direction, Direction::Right));
        assert_eq!(v[0].steps, 10);
    }

    #[test]
    fn read_instructions_multiple() {
        let data = "R5\nL15\nR100\n";
        let mut reader = Cursor::new(data.as_bytes());
        let v = read_instructions(&mut reader).unwrap();
        assert_eq!(v.len(), 3);
        assert!(matches!(v[0].direction, Direction::Right));
        assert_eq!(v[0].steps, 5);
        assert!(matches!(v[1].direction, Direction::Left));
        assert_eq!(v[1].steps, 15);
        assert!(matches!(v[2].direction, Direction::Right));
        assert_eq!(v[2].steps, 100);
    }

    #[test]
    fn read_instructions_invalid_direction() {
        let data = "X10\n";
        let mut reader = Cursor::new(data.as_bytes());
        assert!(read_instructions(&mut reader).is_err());
    }

    #[test]
    fn read_instructions_invalid_step() {
        let data = "R1a\n";
        let mut reader = Cursor::new(data.as_bytes());
        assert!(read_instructions(&mut reader).is_err());
    }
}
