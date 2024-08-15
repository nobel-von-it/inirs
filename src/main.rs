use flags::flags::Args;

mod crates;
mod flags;
mod helpers;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    args.control()?;
    Ok(())
}
