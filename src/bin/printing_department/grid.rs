use std::io::Read;

/// A rectangular grid with each tile either empty or containing a roll.
pub struct Grid {
    content: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn load<R: Read>(mut input: R) -> anyhow::Result<Grid> {
        let mut content = Vec::new();
        input.read_to_end(&mut content)?;

        // XXX We should validate input here:
        //      - check rectangular:
        //          - check all lines have same size
        //          - check height divides input size
        //      - check contains only . or @
        // XXX Assume end-of-line is \n => Does not work for Windows text file.
        let width = content
            .iter()
            .position(|b| *b == b'\n')
            .unwrap_or(content.len());
        // XXX We assume last line has an end-of-line.  Failing this height is
        // off by one.
        let height = content.len() / (width + 1);

        Ok(Self {
            content,
            width,
            height,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_roll_at(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.width());
        debug_assert!(y < self.height());
        let offset = y * (self.width + 1) + x;
        let x = self.content[offset];
        debug_assert!(x == b'@' || x == b'.');
        x == b'@'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::io::Cursor;

    #[test]
    fn load_valid_grid() -> anyhow::Result<()> {
        let payload = indoc! {br"
            ..@@.@@@@.
            @@@.@.@.@@
            @.@.@@@.@.
        "};

        let grid = Grid::load(Cursor::new(payload))?;
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 3);
        assert!(!grid.is_roll_at(0, 0));
        assert!(grid.is_roll_at(2, 0));
        assert!(grid.is_roll_at(0, 1));
        assert!(grid.is_roll_at(1, 1));
        assert!(grid.is_roll_at(0, 2));
        assert!(!grid.is_roll_at(9, 2));
        Ok(())
    }
}
