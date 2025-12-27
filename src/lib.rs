use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Opens file given on first command-line argument.
pub fn open_input_file() -> anyhow::Result<impl BufRead> {
    let path = args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("missing input file"))?;
    let reader = BufReader::new(
        File::open(&path).map_err(|e| anyhow::anyhow!("failed to open {}: {}", path, e))?,
    );
    Ok(reader)
}
