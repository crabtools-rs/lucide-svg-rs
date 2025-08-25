use lucide_svg_rs::{run_cli, Cli, Commands, LucideClient, ICONS_DIR};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_defaults_to_icons_dir() {
    let cli = Cli {
        dir: None,
        command: Commands::List { json: false },
    };
    let result = run_cli(cli).unwrap();
    assert!(result.starts_with("Found "));

    let client = LucideClient::new(ICONS_DIR).unwrap();
    let icons = client.list_icons().unwrap();
    assert!(!icons.is_empty());
}

#[test]
fn test_cli_search_works_from_icons_dir() {
    let client = LucideClient::new(ICONS_DIR).unwrap();
    let icons = client.list_icons().unwrap();
    assert!(!icons.is_empty());
    let first_icon = &icons[0];
    let query = first_icon.chars().take(3).collect::<String>();

    let cli = Cli {
        dir: None,
        command: Commands::Search {
            query: query.clone(),
            json: false,
        },
    };
    let result = run_cli(cli).unwrap();
    assert!(result.starts_with("Found "));

    let matches = client.search_icons(&query).unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_cli_download_all_copies_icons() {
    let tmpdir = tempdir().unwrap();
    let out_path = tmpdir.path().join("exported-icons");
    let out_str = out_path.to_string_lossy().to_string();

    let cli = Cli {
        dir: None,
        command: Commands::DownloadAll {
            out: out_str.clone(),
        },
    };
    let result = run_cli(cli).unwrap();
    assert!(result.contains("Copied"));

    let entries: Vec<_> = fs::read_dir(&out_path).unwrap().collect();
    assert!(!entries.is_empty());
}

#[test]
fn test_cli_list_json_output_is_valid() {
    let cli = Cli {
        dir: None,
        command: Commands::List { json: true },
    };
    let output = run_cli(cli).unwrap();
    let parsed: Vec<String> = serde_json::from_str(&output).unwrap();
    assert!(!parsed.is_empty());
}
