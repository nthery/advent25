// Day 4 - Printing Department

use std::cmp::min;
use std::io::BufRead;

mod grid;
use grid::Grid;

fn main() -> anyhow::Result<()> {
    let input = advent_code_25::open_input_file()?;
    let answer = solve_for(input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn solve_for<R: BufRead>(input: R) -> anyhow::Result<usize> {
    let mut grid = Grid::load(input)?;
    let mut nb_removed_rolls = 0;
    // Compute fixed point.
    loop {
        let n = remove_accessible_rolls(&mut grid, false);
        if n == 0 {
            break
        }
        nb_removed_rolls += n;
    }
    Ok(nb_removed_rolls)
}

/// Removes from `grid` all accessible rolls.
/// Returns number of removed rolls.
fn remove_accessible_rolls(grid: &mut Grid, debug: bool) -> usize {
    let mut sum = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.is_roll_at(x, y) {
                if nb_neighboring_rolls(grid, x, y) < 4 {
                    grid.remove_roll_at(x, y);
                    sum += 1;
                    if debug {
                        print!("x");
                    }
                } else if debug {
                    print!("@");
                }
            } else if debug {
                print!(".");
            }
        }
        if debug {
            println!();
        }
    }
    sum
}

/// Returns number of rolls adjacent to roll at `(x, y)`.
fn nb_neighboring_rolls(grid: &Grid, x_roll: usize, y_roll: usize) -> usize {
    debug_assert!(grid.is_roll_at(x_roll, y_roll));

    let mut nb_rolls = 0;

    for y in y_roll.saturating_sub(1)..=min(y_roll + 1, grid.height() - 1) {
        for x in x_roll.saturating_sub(1)..=min(x_roll + 1, grid.width() - 1) {
            if grid.is_roll_at(x, y) {
                nb_rolls += 1;
            }
        }
    }

    // We counted the roll itself so remove it.
    debug_assert!(nb_rolls >= 1);
    nb_rolls - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::io::Cursor;

    fn new_grid(content: &[u8]) -> Grid {
        Grid::load(Cursor::new(content)).unwrap()
    }

    #[test]
    fn single_accessible_roll() {
        let mut grid = new_grid(indoc! {br"
            ...
            .@.
            ...
        "});
        assert_eq!(remove_accessible_rolls(&mut grid, false), 1);
    }

    #[test]
    fn corners_are_acessible() {
        let mut grid = new_grid(indoc! {br"
            @.@
            .@.
            @.@
        "});
        assert_eq!(remove_accessible_rolls(&mut grid, false), 5);
    }

    #[test]
    fn no_neighbors() {
        let grid = new_grid(indoc! {br"
            ...
            .@.
            ...
        "});
        assert_eq!(nb_neighboring_rolls(&grid, 1, 1), 0);
    }

    #[test]
    fn full_neighbors() {
        let grid = new_grid(indoc! {br"
            @@@
            @@@
            @@@
        "});
        assert_eq!(nb_neighboring_rolls(&grid, 1, 1), 8);
    }

    #[test]
    fn borders() {
        let grid = new_grid(indoc! {br"
            @@@
            @.@
            @@@
        "});
        assert_eq!(nb_neighboring_rolls(&grid, 0, 0), 2);
        assert_eq!(nb_neighboring_rolls(&grid, 1, 0), 4);
        assert_eq!(nb_neighboring_rolls(&grid, 2, 0), 2);
        assert_eq!(nb_neighboring_rolls(&grid, 1, 0), 4);
        assert_eq!(nb_neighboring_rolls(&grid, 1, 2), 4);
        assert_eq!(nb_neighboring_rolls(&grid, 0, 2), 2);
        assert_eq!(nb_neighboring_rolls(&grid, 1, 2), 4);
        assert_eq!(nb_neighboring_rolls(&grid, 2, 2), 2);
    }
}
