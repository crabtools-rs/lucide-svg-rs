use lucide_svg_rs::{run_cli, Cli, Commands, ICONS_DIR};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Using ICONS_DIR = {ICONS_DIR}");
    let cli = Cli {
        dir: None,
        command: Commands::Search {
            query: "alert".into(),
            json: false,
        },
    };
    println!("{}", run_cli(cli)?);
    Ok(())
}
