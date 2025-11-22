use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use std::{
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use tar::Archive;

/// Built-in icons tar.gz file shipped with this crate
pub const ICONS_TAR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/icons.tar.gz");

/// Offline, file-based Lucide client.
#[derive(Debug, Default)]
pub struct LucideClient {
    tar_path: PathBuf,
}

impl LucideClient {
    /// Initialize with a local tar.gz file containing `*.svg`
    pub fn new<P: AsRef<Path>>(tar_path: P) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tar_path: tar_path.as_ref().to_path_buf(),
        })
    }

    /// List all icons (names without `.svg`), sorted
    pub fn list_icons(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let file = fs::File::open(&self.tar_path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        let mut icons = Vec::new();
        for entry in archive.entries()? {
            let entry = entry?;
            let path = entry.path()?;
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.ends_with(".svg") {
                        if let Some(stem) = Path::new(name_str).file_stem() {
                            icons.push(stem.to_string_lossy().to_string());
                        }
                    }
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

    /// Get the SVG contents for a given icon name without the `.svg` extension.
    pub fn get_icon_content(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let file_name = if name.ends_with(".svg") {
            name.to_string()
        } else {
            format!("{name}.svg")
        };
        let file = fs::File::open(&self.tar_path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            if let Some(entry_name) = path.file_name() {
                if entry_name == file_name.as_str() {
                    let mut content = String::new();
                    entry.read_to_string(&mut content)?;
                    return Ok(content);
                }
            }
        }
        Err(format!("Icon '{}' not found", name).into())
    }

    /// Extract all SVG files into target dir. Returns (total_found, failed_names)
    pub fn download_all_icons<P: AsRef<Path>>(
        &self,
        target_dir: P,
    ) -> Result<(usize, Vec<String>), Box<dyn Error>> {
        let mut failed = Vec::new();
        let target_dir = target_dir.as_ref();
        fs::create_dir_all(target_dir)?;
        let file = fs::File::open(&self.tar_path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        let mut total = 0;
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.to_path_buf();
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.ends_with(".svg") {
                        total += 1;
                        let dst = target_dir.join(file_name);
                        if let Err(_) = entry.unpack(&dst) {
                            failed.push(name_str.to_string());
                        }
                    }
                } else {
                    failed.push(format!("Invalid filename: {}", path.display()));
                }
            }
        }
        Ok((total, failed))
    }
}

/// CLI parser
#[derive(Parser, Debug)]
#[command(name = "lucide-svg-rs")]
#[command(about = "Work with Lucide icons from a local tar.gz file", long_about = None)]
pub struct Cli {
    /// Path to local SVG tar.gz file (defaults to built-in icons.tar.gz)
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
    let tar_path = cli.dir.clone().unwrap_or_else(|| ICONS_TAR.to_string());
    let client = LucideClient::new(&tar_path)?;
    Ok(match cli.command {
        Commands::List { json } => {
            if json {
                client.list_icons_json()?
            } else {
                let icons = client.list_icons()?;
                format!("Found {}\n{icons:?}", icons.len())
            }
        }
        Commands::Search { query, json } => {
            if json {
                client.search_icons_json(&query)?
            } else {
                let matches = client.search_icons(&query)?;
                format!("Found {}\n{matches:?}", matches.len())
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
// use lucide_svg_rs::{LucideClient, ICONS_TAR};
// let client = LucideClient::new(ICONS_TAR).unwrap();
// let icons = client.list_icons().unwrap();
// assert!(!icons.is_empty());
// ```
