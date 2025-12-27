const MAX_DIAL: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dial {
    pos: usize,
}

impl Dial {
    pub fn new(pos: usize) -> Self {
        debug_assert!(pos < MAX_DIAL);
        Self { pos }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn turn_right(&mut self, steps: usize) {
        self.pos = (self.pos + steps) % MAX_DIAL;
        debug_assert!(self.pos < MAX_DIAL);
    }

    pub fn turn_left(&mut self, steps: usize) {
        let steps = steps % MAX_DIAL;
        if self.pos < steps {
            self.pos = MAX_DIAL - (steps - self.pos);
        } else {
            self.pos -= steps;
        }
        debug_assert!(self.pos < MAX_DIAL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_rotation_no_wrap() {
        let mut d = Dial::new(10);
        d.turn_right(5);
        assert_eq!(d.position(), 15);
    }

    #[test]
    fn right_rotation_wrap() {
        let mut d = Dial::new(60);
        d.turn_right(60);
        assert_eq!(d.position(), 20);
    }

    #[test]
    fn left_rotation_no_underflow() {
        let mut d = Dial::new(20);
        d.turn_left(5);
        assert_eq!(d.position(), 15);
    }

    #[test]
    fn left_rotation_equal_steps() {
        let mut d = Dial::new(10);
        d.turn_left(10);
        assert_eq!(d.position(), 0);
    }

    #[test]
    fn left_rotation_underflow() {
        let mut d = Dial::new(10);
        d.turn_left(30);
        assert_eq!(d.position(), 80);
    }

    #[test]
    fn right_rotation_steps_greater_than_max_dial() {
        let mut d = Dial::new(10);
        d.turn_right(260);
        assert_eq!(d.position(), 70);
    }

    #[test]
    fn left_rotation_steps_greater_than_max_dial() {
        let mut d = Dial::new(10);
        d.turn_left(130);
        assert_eq!(d.position(), 80);
    }

    #[test]
    fn left_rotation_steps_greater_than_max_dial_ends_on_zero() {
        let mut d = Dial::new(49);
        d.turn_left(849);
        assert_eq!(d.position(), 0);
    }
}
