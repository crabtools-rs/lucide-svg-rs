use clap::Parser;
use lucide_svg_rs::{run_cli, Cli};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("{}", run_cli(cli)?);
    Ok(())
}
