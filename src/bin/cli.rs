use anyhow::Result;
use clap::{Parser, Subcommand};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use lucide_svg_rs::LucideClient;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "lucide-cli")]
#[command(about = "A CLI tool to download Lucide SVG icons with preview")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available icons
    List {
        /// Search pattern to filter icons
        #[arg(short, long)]
        search: Option<String>,
        /// Number of icons to display (default: all)
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Download specific icons with interactive selection
    Select {
        /// Output directory
        #[arg(short, long, default_value = "lucide_icons")]
        output: PathBuf,
        /// Search pattern to filter icons before selection
        #[arg(short, long)]
        search: Option<String>,
    },
    /// Download specific icons by name
    Download {
        /// Icon names to download (without .svg extension)
        icons: Vec<String>,
        /// Output directory
        #[arg(short, long, default_value = "lucide_icons")]
        output: PathBuf,
    },
    /// Download all icons
    All {
        /// Output directory
        #[arg(short, long, default_value = "lucide_icons")]
        output: PathBuf,
    },
    /// Preview an icon's SVG content
    Preview {
        /// Icon name to preview (without .svg extension)
        icon: String,
    },
    /// Search for icons interactively
    Search,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = LucideClient::new();

    match cli.command {
        Commands::List { search, limit } => {
            list_icons(&client, search.as_deref(), limit)?;
        }
        Commands::Select { output, search } => {
            interactive_select(&client, &output, search.as_deref())?;
        }
        Commands::Download { icons, output } => {
            download_icons(&client, &icons, &output)?;
        }
        Commands::All { output } => {
            download_all_icons(&client, &output)?;
        }
        Commands::Preview { icon } => {
            preview_icon(&client, &icon)?;
        }
        Commands::Search => {
            interactive_search(&client)?;
        }
    }

    Ok(())
}

fn list_icons(client: &LucideClient, search: Option<&str>, limit: Option<usize>) -> Result<()> {
    let icons = if let Some(pattern) = search {
        client.search_icons(pattern)?
    } else {
        client.list_icons()?
    };

    let display_icons = if let Some(limit) = limit {
        icons.into_iter().take(limit).collect()
    } else {
        icons
    };

    println!("{}", style("Available Icons:").bold().blue());
    println!("{}", style("================").blue());

    for (index, icon) in display_icons.iter().enumerate() {
        let name_without_ext = icon.name.strip_suffix(".svg").unwrap_or(&icon.name);
        let size_kb = icon.size as f64 / 1024.0;
        println!(
            "{:3}. {} {}",
            index + 1,
            style(name_without_ext).cyan(),
            style(format!("({size_kb:.1}KB)")).dim()
        );
    }

    println!("\nTotal: {} icons", display_icons.len());
    Ok(())
}

fn interactive_select(client: &LucideClient, output: &Path, search: Option<&str>) -> Result<()> {
    let icons = if let Some(pattern) = search {
        client.search_icons(pattern)?
    } else {
        client.list_icons()?
    };

    if icons.is_empty() {
        println!("No icons found!");
        return Ok(());
    }

    let items: Vec<String> = icons
        .iter()
        .map(|icon| {
            let name_without_ext = icon.name.strip_suffix(".svg").unwrap_or(&icon.name);
            let size_kb = icon.size as f64 / 1024.0;
            format!("{name_without_ext} ({size_kb:.1}KB)")
        })
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select icons to download (use space to select, enter to confirm)")
        .items(&items)
        .interact()?;

    if selections.is_empty() {
        println!("No icons selected!");
        return Ok(());
    }

    let selected_icons: Vec<&str> = selections
        .iter()
        .map(|&i| icons[i].name.strip_suffix(".svg").unwrap_or(&icons[i].name))
        .collect();

    download_icons(
        client,
        &selected_icons
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        output,
    )?;
    Ok(())
}

fn download_icons(client: &LucideClient, icon_names: &[String], output: &Path) -> Result<()> {
    if icon_names.is_empty() {
        println!("No icons specified!");
        return Ok(());
    }

    println!("{}", style("Downloading selected icons...").bold().green());

    // Add .svg extension if not present
    let full_names: Vec<&str> = icon_names
        .iter()
        .map(|name| {
            if name.ends_with(".svg") {
                name.as_str()
            } else {
                // We'll need to look up the full name
                name.as_str()
            }
        })
        .collect();

    let results = client.download_icons(&full_names, output)?;

    let mut successful = 0;
    let mut failed = 0;

    for (name, result) in results {
        match result {
            Ok(_) => {
                println!("✓ {}", style(name).green());
                successful += 1;
            }
            Err(e) => {
                println!("✗ {}: {}", style(name).red(), e);
                failed += 1;
            }
        }
    }

    println!("\n{}", style("Download Summary:").bold());
    println!("Successfully downloaded: {}", style(successful).green());
    if failed > 0 {
        println!("Failed: {}", style(failed).red());
    }
    println!("Output directory: {}", style(output.display()).cyan());

    Ok(())
}

fn download_all_icons(client: &LucideClient, output: &Path) -> Result<()> {
    println!(
        "{}",
        style("Downloading all Lucide icons...").bold().green()
    );
    println!("This may take a while...\n");

    let (successful, failed) = client.download_all_icons(output)?;

    println!("\n{}", style("Download Complete!").bold().green());
    println!("Successfully downloaded: {}", style(successful).green());
    if !failed.is_empty() {
        println!("Failed downloads: {}", style(failed.len()).red());
        for failure in failed {
            println!("  - {}", style(failure).red());
        }
    }
    println!("Output directory: {}", style(output.display()).cyan());

    Ok(())
}

fn preview_icon(client: &LucideClient, icon_name: &str) -> Result<()> {
    let full_name = if icon_name.ends_with(".svg") {
        icon_name.to_string()
    } else {
        format!("{icon_name}.svg")
    };

    println!(
        "{}",
        style(format!("Previewing: {full_name}")).bold().blue()
    );
    println!("{}", style("=".repeat(50)).blue());

    match client.get_icon_content(&full_name) {
        Ok(content) => {
            // Display SVG attributes
            if let Some(svg_start) = content.find("<svg") {
                if let Some(svg_end) = content[svg_start..].find(">") {
                    let svg_tag = &content[svg_start..svg_start + svg_end + 1];
                    println!("{}", style("SVG Tag:").bold());
                    println!("{}\n", style(svg_tag).dim());
                }
            }

            // Show content with syntax highlighting (basic)
            println!("{}", style("Content:").bold());
            for line in content.lines() {
                if line.trim().starts_with('<') {
                    println!("{}", style(line).cyan());
                } else {
                    println!("{line}");
                }
            }

            println!(
                "\n{}",
                style(format!("Size: {} bytes", content.len())).dim()
            );
        }
        Err(e) => {
            println!("{}: {}", style("Error").red(), e);
        }
    }

    Ok(())
}

fn interactive_search(client: &LucideClient) -> Result<()> {
    let term = Term::stdout();

    loop {
        print!("{}", style("🔍 Search icons (or 'quit' to exit): ").bold());
        let input = dialoguer::Input::<String>::new().interact_text()?;

        if input.trim().to_lowercase() == "quit" {
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        let results = client.search_icons(&input)?;

        if results.is_empty() {
            println!("{}", style("No icons found!").yellow());
            continue;
        }

        println!("\n{} {}", style("Found:").bold().green(), results.len());

        let items: Vec<String> = results
            .iter()
            .map(|icon| {
                let name_without_ext = icon.name.strip_suffix(".svg").unwrap_or(&icon.name);
                name_without_ext.to_string()
            })
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an icon to preview (or press Esc)")
            .items(&items)
            .default(0)
            .interact_opt()?;

        if let Some(index) = selection {
            term.clear_screen()?;
            preview_icon(client, &items[index])?;

            let download = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Download this icon?")
                .default(false)
                .interact()?;

            if download {
                download_icons(
                    client,
                    &[items[index].clone()],
                    &PathBuf::from("lucide_icons"),
                )?;
            }
        }

        println!("\n{}", style("-".repeat(50)).dim());
    }

    Ok(())
}
