use clap::{Parser, Subcommand};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

/// Built-in icons directory shipped with this crate
pub const ICONS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/icons");

/// Offline, file-based Lucide client.
pub struct LucideClient {
    svg_dir: PathBuf,
}

impl LucideClient {
    /// Initialize with a local directory containing `*.svg`
    pub fn new<P: AsRef<Path>>(svg_dir: P) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            svg_dir: svg_dir.as_ref().to_path_buf(),
        })
    }

    /// List all icons (names without `.svg`), sorted
    pub fn list_icons(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut icons = Vec::new();
        for entry in fs::read_dir(&self.svg_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|ext| ext == "svg").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    icons.push(stem.to_string_lossy().to_string());
                }
            }
        }
        icons.sort();
        Ok(icons)
    }

    /// List all icons as pretty JSON
    pub fn list_icons_json(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string_pretty(&self.list_icons()?)?)
    }

    /// Search icons by substring (case-sensitive)
    pub fn search_icons(&self, query: &str) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .list_icons()?
            .into_iter()
            .filter(|n| n.contains(query))
            .collect())
    }

    pub fn search_icons_json(&self, query: &str) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string_pretty(&self.search_icons(query)?)?)
    }

    /// Copy all SVG files into target dir. Returns (total_found, failed_names)
    pub fn download_all_icons<P: AsRef<Path>>(
        &self,
        target_dir: P,
    ) -> Result<(usize, Vec<String>), Box<dyn Error>> {
        let mut failed = Vec::new();
        let target_dir = target_dir.as_ref();
        fs::create_dir_all(target_dir)?;
        let mut total = 0;
        for entry in fs::read_dir(&self.svg_dir)? {
            let entry = entry?;
            let src = entry.path();
            if src.extension().map(|ext| ext == "svg").unwrap_or(false) {
                total += 1;
                let dst = target_dir.join(src.file_name().unwrap());
                if fs::copy(&src, &dst).is_err() {
                    failed.push(src.file_name().unwrap().to_string_lossy().to_string());
                }
            }
        }
        Ok((total, failed))
    }
}

/// CLI parser
#[derive(Parser, Debug)]
#[command(name = "lucide-cli")]
#[command(about = "Work with Lucide icons from a local directory", long_about = None)]
pub struct Cli {
    /// Path to local SVG directory (defaults to built-in icons)
    #[arg(short, long)]
    pub dir: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

/// CLI subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all icons
    List {
        #[arg(long)]
        json: bool,
    },
    /// Search by substring
    Search {
        query: String,
        #[arg(long)]
        json: bool,
    },
    /// Copy all icons to a directory
    DownloadAll { out: String },
}

/// Run CLI and return printable output
pub fn run_cli(cli: Cli) -> Result<String, Box<dyn Error>> {
    let dir = cli.dir.clone().unwrap_or_else(|| ICONS_DIR.to_string());
    let client = LucideClient::new(&dir)?;
    Ok(match cli.command {
        Commands::List { json } => {
            if json {
                client.list_icons_json()?
            } else {
                format!("Found {} icons", client.list_icons()?.len())
            }
        }
        Commands::Search { query, json } => {
            if json {
                client.search_icons_json(&query)?
            } else {
                format!("Found {} matches", client.search_icons(&query)?.len())
            }
        }
        Commands::DownloadAll { out } => {
            let (c, f) = client.download_all_icons(&out)?;
            format!("Copied {} icons ({} failed)", c, f.len())
        }
    })
}

// # Examples
//
// ```
// use lucide_svg_rs::{LucideClient, ICONS_DIR};
// let client = LucideClient::new(ICONS_DIR).unwrap();
// let icons = client.list_icons().unwrap();
// assert!(!icons.is_empty());
// ```
