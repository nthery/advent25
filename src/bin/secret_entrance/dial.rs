const MAX_DIAL: usize = 100;

/// A round dial graduated from 0 to 99 clock-wise.
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

    /// Turns dial `n` clicks to the right and returns number of times dial pointed at zero
    /// while being turned.
    pub fn turn_right(&mut self, n: usize) -> usize {
        let nb_dial_on_zero = (self.pos + n) / MAX_DIAL;
        self.pos = (self.pos + n) % MAX_DIAL;
        debug_assert!(self.pos < MAX_DIAL);
        nb_dial_on_zero
    }

    /// Turns dial `n` clicks to the left and returns number of times dial pointed at zero
    /// while being turned.
    pub fn turn_left(&mut self, n: usize) -> usize {
        // To avoid complications induced by negative numbers, create a mirror
        // dial, turn it right instead, and mirror resulting position back.
        let mut mirror_dial = Dial::new(Self::mirror_position(self.pos));
        let nb_dial_on_zero = mirror_dial.turn_right(n);
        self.pos = Self::mirror_position(mirror_dial.position());
        debug_assert!(self.pos < MAX_DIAL);
        nb_dial_on_zero
    }

    /// Returns dial position `pos` in a mirror dial.
    fn mirror_position(pos: usize) -> usize {
        if pos == 0 { 0 } else { MAX_DIAL - pos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_rotation_no_wrap() {
        let mut d = Dial::new(10);
        assert_eq!(d.turn_right(5), 0);
        assert_eq!(d.position(), 15);
    }

    #[test]
    fn right_rotation_wrap_once() {
        let mut d = Dial::new(60);
        assert_eq!(d.turn_right(60), 1);
        assert_eq!(d.position(), 20);
    }

    #[test]
    fn right_rotation_wrap_many() {
        let mut d = Dial::new(60);
        assert_eq!(d.turn_right(260), 3);
        assert_eq!(d.position(), 20);
    }

    #[test]
    fn right_rotation_end_on_zero() {
        let mut d = Dial::new(1);
        assert_eq!(d.turn_right(99), 1);
        assert_eq!(d.position(), 0);
    }

    #[test]
    fn right_rotation_wrap_and_end_on_zero() {
        let mut d = Dial::new(0);
        assert_eq!(d.turn_right(200), 2);
        assert_eq!(d.position(), 0);
    }

    #[test]
    fn left_rotation_no_wrap() {
        let mut d = Dial::new(20);
        assert_eq!(d.turn_left(5), 0);
        assert_eq!(d.position(), 15);
    }

    #[test]
    fn left_rotation_end_on_zero() {
        let mut d = Dial::new(10);
        assert_eq!(d.turn_left(10), 1);
        assert_eq!(d.position(), 0);
    }

    #[test]
    fn left_rotation_wrap_once() {
        let mut d = Dial::new(10);
        assert_eq!(d.turn_left(30), 1);
        assert_eq!(d.position(), 80);
    }

    #[test]
    fn left_rotation_wrap_many() {
        let mut d = Dial::new(10);
        assert_eq!(d.turn_left(130), 2);
        assert_eq!(d.position(), 80);
    }

    #[test]
    fn left_rotation_wrap_many_end_on_zero() {
        let mut d = Dial::new(49);
        assert_eq!(d.turn_left(849), 9);
        assert_eq!(d.position(), 0);
    }
}
