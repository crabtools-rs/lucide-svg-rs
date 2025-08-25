use lucide_svg_rs::{run_cli, Cli, Commands, ICONS_DIR};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Using ICONS_DIR = {ICONS_DIR}");
    let out = env::args()
        .nth(1)
        .unwrap_or_else(|| "exported-icons".into());
    let cli = Cli {
        dir: None,
        command: Commands::DownloadAll { out: out.clone() },
    };
    println!("{}", run_cli(cli)?);
    Ok(())
}
