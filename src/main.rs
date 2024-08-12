mod crates;
mod flags;
mod helpers;
mod opt;
mod snippets;

use clap::Parser;

use crate::flags::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    args.control()?;
    Ok(())
}
