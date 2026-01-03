// Day 4 - Printing Department

use std::io::BufRead;

fn main() -> anyhow::Result<()> {
    let input = advent_code_25::open_input_file()?;
    let answer = solve_for(input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn solve_for<R: BufRead>(_input: R) -> anyhow::Result<usize> {
    todo!()
}
