use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

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
    Ok(execute_instructions(&instructions))
}

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    steps: usize,
}

const MAX_DIAL: usize = 100;

fn read_instructions<R: BufRead>(input: &mut R) -> anyhow::Result<Vec<Rotation>> {
    let mut instructions = Vec::new();
    for (i, line) in input.lines().enumerate() {
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
            _ => anyhow::bail!("line {}: invalid direction: {}", i, direction_char),
        };
        let steps_str = &line[1..];
        let steps: usize = steps_str
            .parse()
            .map_err(|e| anyhow::anyhow!("line {}: invalid step: {}: {}", i, steps_str, e))?;
        instructions.push(Rotation { direction, steps });
    }
    Ok(instructions)
}

fn execute_instructions(instructions: &[Rotation]) -> usize {
    let mut dial = 50;
    let mut number_of_zeroes = 0;
    debug_assert!(dial != 0);
    for rotation in instructions {
        dial = rotate(dial, rotation);
        if dial == 0 {
            number_of_zeroes += 1;
        }
    }
    number_of_zeroes
}

fn rotate(mut dial: usize, rotation: &Rotation) -> usize {
    debug_assert!(dial < MAX_DIAL);
    match rotation.direction {
        Direction::Right => {
            dial = (dial + rotation.steps) % MAX_DIAL;
        }
        Direction::Left => {
            if dial < rotation.steps {
                dial = MAX_DIAL - (rotation.steps - dial) % MAX_DIAL;
            } else {
                dial -= rotation.steps;
            }
        }
    };
    debug_assert!(dial < MAX_DIAL);
    dial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_rotation_no_wrap() {
        let r = Rotation {
            direction: Direction::Right,
            steps: 5,
        };
        assert_eq!(rotate(10, &r), 15);
    }

    #[test]
    fn right_rotation_wrap() {
        let r = Rotation {
            direction: Direction::Right,
            steps: 60,
        };
        assert_eq!(rotate(60, &r), 20);
    }

    #[test]
    fn left_rotation_no_underflow() {
        let r = Rotation {
            direction: Direction::Left,
            steps: 5,
        };
        assert_eq!(rotate(20, &r), 15);
    }

    #[test]
    fn left_rotation_equal_steps() {
        let r = Rotation {
            direction: Direction::Left,
            steps: 10,
        };
        assert_eq!(rotate(10, &r), 0);
    }

    #[test]
    fn left_rotation_underflow() {
        let r = Rotation {
            direction: Direction::Left,
            steps: 30,
        };
        assert_eq!(rotate(10, &r), 80);
    }

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
