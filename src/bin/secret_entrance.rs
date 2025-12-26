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
    Ok(execute_instructions(&instructions))
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

const MAX_DIAL: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dial {
    pos: usize,
}

impl Dial {
    fn new(pos: usize) -> Self {
        debug_assert!(pos < MAX_DIAL);
        Self { pos }
    }

    fn position(&self) -> usize {
        self.pos
    }

    fn turn_right(&mut self, steps: usize) {
        self.pos = (self.pos + steps) % MAX_DIAL;
        debug_assert!(self.pos < MAX_DIAL);
    }

    fn turn_left(&mut self, steps: usize) {
        let steps = steps % MAX_DIAL;
        if self.pos < steps {
            self.pos = MAX_DIAL - (steps - self.pos);
        } else {
            self.pos -= steps;
        }
        debug_assert!(self.pos < MAX_DIAL);
    }
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

fn execute_instructions(instructions: &[Rotation]) -> usize {
    let mut dial = Dial::new(50);
    let mut number_of_zeroes = 0;
    debug_assert!(dial.position() != 0);
    for rotation in instructions {
        match rotation.direction {
            Direction::Right => dial.turn_right(rotation.steps),
            Direction::Left => dial.turn_left(rotation.steps),
        }
        if dial.position() == 0 {
            number_of_zeroes += 1;
        }
    }
    number_of_zeroes
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
        let mut d = Dial::new(10);
        d.turn_right(r.steps);
        assert_eq!(d.position(), 15);
    }

    #[test]
    fn right_rotation_wrap() {
        let r = Rotation {
            direction: Direction::Right,
            steps: 60,
        };
        let mut d = Dial::new(60);
        d.turn_right(r.steps);
        assert_eq!(d.position(), 20);
    }

    #[test]
    fn left_rotation_no_underflow() {
        let r = Rotation {
            direction: Direction::Left,
            steps: 5,
        };
        let mut d = Dial::new(20);
        d.turn_left(r.steps);
        assert_eq!(d.position(), 15);
    }

    #[test]
    fn left_rotation_equal_steps() {
        let r = Rotation {
            direction: Direction::Left,
            steps: 10,
        };
        let mut d = Dial::new(10);
        d.turn_left(r.steps);
        assert_eq!(d.position(), 0);
    }

    #[test]
    fn left_rotation_underflow() {
        let r = Rotation {
            direction: Direction::Left,
            steps: 30,
        };
        let mut d = Dial::new(10);
        d.turn_left(r.steps);
        assert_eq!(d.position(), 80);
    }

    #[test]
    fn right_rotation_steps_greater_than_max_dial() {
        let r = Rotation {
            direction: Direction::Right,
            steps: 260,
        };
        let mut d = Dial::new(10);
        d.turn_right(r.steps);
        assert_eq!(d.position(), 70);
    }

    #[test]
    fn left_rotation_steps_greater_than_max_dial() {
        let l = Rotation {
            direction: Direction::Left,
            steps: 130,
        };
        let mut d = Dial::new(10);
        d.turn_left(l.steps);
        assert_eq!(d.position(), 80);
    }

    #[test]
    fn left_rotation_steps_greater_than_max_dial_ends_on_zero() {
        let l = Rotation {
            direction: Direction::Left,
            steps: 849,
        };
        let mut d = Dial::new(49);
        d.turn_left(l.steps);
        assert_eq!(d.position(), 0);
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
