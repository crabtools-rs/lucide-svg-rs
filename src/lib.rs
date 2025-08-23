use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LucideError {
    #[error("Network error: {0}")]
    Network(#[from] ureq::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Icon not found: {0}")]
    IconNotFound(String),
}

/// Information about the icon for the purpose of caching metadata (not the same as data caching)
/// and speeding internal searches without downloading actual SVG data.
#[derive(Debug, Clone)]
pub struct IconInfo {
    pub name: String,
    pub download_url: String,
    pub size: u64,
}

pub struct LucideClient {
    user_agent: String,
}

impl Default for LucideClient {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_icon_info(name: &str) -> Option<IconInfo> {
    let icon_info = IconInfo {
        name: String::from(name),
        download_url: String::from(""),
        size: 0,
    };

    Some(icon_info)
}

impl LucideClient {
    /// Instanciate a LucideClient with the default user agent.
    pub fn new() -> Self {
        Self {
            user_agent: "lucide-svg-rs/0.1".to_string(),
        }
    }

    /// Instanciate a LucideClient with the provided user agent.
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Fetch the list of available SVG icons from Lucide repository
    pub fn list_icons(&self) -> Result<Vec<IconInfo>, LucideError> {
        let api_url = "https://api.github.com/repos/lucide-icons/lucide/contents/icons";

        let body = ureq::get(api_url)
            .header("User-Agent", &self.user_agent)
            .call()?
            .body_mut()
            .read_to_string()
            .map_err(LucideError::Network)?;

        let files: Vec<Value> = serde_json::from_str(&body)?;

        let mut icons = Vec::new();

        for file in files {
            if let (Some(name), Some(download_url), Some(file_type), Some(size)) = (
                file["name"].as_str(),
                file["download_url"].as_str(),
                file["type"].as_str(),
                file["size"].as_u64(),
            ) {
                if file_type == "file" && name.ends_with(".svg") {
                    icons.push(IconInfo {
                        name: name.to_string(),
                        download_url: download_url.to_string(),
                        size,
                    });
                }
            }
        }

        // Sort icons alphabetically
        icons.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(icons)
    }

    /// Download a single SVG icon by name
    pub fn download_icon(
        &self,
        icon_name: &str,
        output_path: &Path,
    ) -> Result<String, LucideError> {
        let icons = self.list_icons()?;
        let icon = icons
            .iter()
            // .inspect(|i| println!("{i:?}"))
            .find(|i| i.name == icon_name)
            .ok_or_else(|| LucideError::IconNotFound(icon_name.to_string()))?;

        self.download_icon_from_url(&icon.download_url, output_path)
    }

    /// Download multiple SVG icons by names
    pub fn download_icons(
        &self,
        icon_names: &[&str],
        output_dir: &Path,
    ) -> Result<HashMap<String, Result<String, LucideError>>, LucideError> {
        // Ensure output directory exists
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let icons = self.list_icons()?;
        let mut results = HashMap::new();

        for &icon_name in icon_names {
            let result = if let Some(icon) = icons.iter().find(|i| i.name == icon_name) {
                let output_path = output_dir.join(&icon.name);
                self.download_icon_from_url(&icon.download_url, &output_path)
            } else {
                Err(LucideError::IconNotFound(icon_name.to_string()))
            };

            results.insert(icon_name.to_string(), result);
        }

        Ok(results)
    }

    /// Download all available SVG icons
    pub fn download_all_icons(
        &self,
        output_dir: &Path,
    ) -> Result<(usize, Vec<String>), LucideError> {
        let icons = self.list_icons()?;

        // Ensure output directory exists
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let mut successful_downloads = 0;
        let mut failed_downloads = Vec::new();

        for (index, icon) in icons.iter().enumerate() {
            let output_path = output_dir.join(&icon.name);

            match self.download_icon_from_url(&icon.download_url, &output_path) {
                Ok(_) => {
                    successful_downloads += 1;
                    if (index + 1) % 50 == 0 {
                        println!("Progress: {}/{} icons downloaded", index + 1, icons.len());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to download {}: {}", icon.name, e);
                    failed_downloads.push(format!("{}: {}", icon.name, e));
                }
            }
        }

        Ok((successful_downloads, failed_downloads))
    }

    /// Get SVG content as string without saving to file
    pub fn get_icon_content(&self, icon_name: &str) -> Result<String, LucideError> {
        let icons = self.list_icons()?;
        let icon = icons
            .iter()
            .find(|i| i.name == icon_name)
            .ok_or_else(|| LucideError::IconNotFound(icon_name.to_string()))?;

        ureq::get(&icon.download_url)
            .header("User-Agent", &self.user_agent)
            .call()?
            .body_mut()
            .read_to_string()
            .map_err(LucideError::Network)
    }

    /// Search for icons by name pattern
    pub fn search_icons(&self, pattern: &str) -> Result<Vec<IconInfo>, LucideError> {
        let icons = self.list_icons()?;
        let pattern_lower = pattern.to_lowercase();

        let filtered: Vec<IconInfo> = icons
            .into_iter()
            .filter(|icon| {
                let name_without_ext = icon.name.strip_suffix(".svg").unwrap_or(&icon.name);
                name_without_ext.to_lowercase().contains(&pattern_lower)
            })
            .collect();

        Ok(filtered)
    }

    fn download_icon_from_url(&self, url: &str, file_path: &Path) -> Result<String, LucideError> {
        let svg_content = ureq::get(url)
            .header("User-Agent", &self.user_agent)
            .call()?
            .body_mut()
            .read_to_string()
            .map_err(LucideError::Network)?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Path already exists");
        }

        let mut file = fs::File::create(file_path)?;
        file.write_all(svg_content.as_bytes())?;

        Ok(svg_content)
    }
}

#[cfg(test)]
mod tests {
    use crate::LucideClient;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_list_icons() {
        let client = LucideClient::new();
        let icons = client.list_icons().unwrap();

        assert!(!icons.is_empty());
        assert!(icons.iter().any(|icon| icon.name == "book-heart.svg"));
    }

    #[test]
    fn test_search_icons() {
        let client = LucideClient::new();
        let results = client.search_icons("book-heart").unwrap();

        assert!(!results.is_empty());
        assert!(results.iter().any(|icon| icon.name.contains("book-heart")));
    }

    #[test]
    fn test_download_single_icon() {
        let client = LucideClient::new();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("book-heart.svg");

        let result = client.download_icon("book-heart.svg", &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("<svg"));
    }

    #[test]
    fn test_get_icon_content() {
        let client = LucideClient::new();
        let content = client.get_icon_content("book-heart.svg").unwrap();

        assert!(content.contains("<svg"));
        assert!(content.contains("</svg>"));
    }
}
