mod crates;
mod flags;
mod snippets;

use clap::Parser;

use crate::flags::Args;

fn main() {
    let args = Args::parse();
    args.create_by_name();
    args.add_all_crates();
    args.build();
}
