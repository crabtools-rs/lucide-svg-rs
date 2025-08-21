# Lucide SVG Fetcher

A Rust library and CLI tool for downloading SVG icons from the [Lucide](https://lucide.dev/) icon library with interactive selection and preview capabilities.

## Features

### Library Features

- 📦 **Easy Integration**: Simple API for downloading Lucide SVG icons
- 🔍 **Search Functionality**: Search for icons by name patterns
- 📋 **Batch Downloads**: Download multiple icons at once
- 🎯 **Single Icon Downloads**: Download specific icons by name
- 💾 **Content Retrieval**: Get SVG content as strings without saving to disk
- 🔧 **Customizable**: Configure user agents and other settings

### CLI Features

- 🖥️ **Interactive Selection**: Multi-select interface for choosing icons
- 👁️ **Preview Mode**: View SVG content before downloading
- 🔍 **Interactive Search**: Real-time search with preview
- 📊 **Progress Tracking**: Download progress and statistics
- 🎨 **Colored Output**: Beautiful terminal interface with colors and styling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lucide-svg-fetcher = "0.1.0"
```

Or install the CLI tool:

```bash
cargo install lucide-svg-fetcher
```

## Library Usage

### Basic Example

```rust
use lucide_svg_fetcher::LucideClient;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LucideClient::new();

    // List all available icons
    let icons = client.list_icons()?;
    println!("Found {} icons", icons.len());

    // Search for specific icons
    let heart_icons = client.search_icons("heart")?;

    // Download specific icons
    let icons_to_download = ["heart.svg", "home.svg", "user.svg"];
    let results = client.download_icons(&icons_to_download, Path::new("icons"))?;

    // Get icon content without saving
    let svg_content = client.get_icon_content("heart.svg")?;

    Ok(())
}
```

### Advanced Usage

```rust
use lucide_svg_fetcher::LucideClient;
use std::path::Path;

// Create client with custom user agent
let client = LucideClient::new()
    .with_user_agent("my-app/1.0".to_string());

// Download all icons (be careful - this downloads 1000+ icons!)
let (successful, failed) = client.download_all_icons(Path::new("all_icons"))?;

// Download single icon
client.download_icon("star.svg", Path::new("icons/star.svg"))?;
```

## CLI Usage

### List Icons

```bash
# List all icons
lucide-cli list

# Search for specific icons
lucide-cli list --search heart

# Limit number of results
lucide-cli list --search arrow --limit 10
```

### Interactive Selection

```bash
# Interactive multi-select interface
lucide-cli select

# Pre-filter with search before selection
lucide-cli select --search navigation --output ./my-icons
```

### Download Specific Icons

```bash
# Download specific icons by name
lucide-cli download heart home user --output ./icons

# Download to custom directory
lucide-cli download star moon sun --output ./weather-icons
```

### Preview Icons

```bash
# Preview an icon's SVG content
lucide-cli preview heart

# Preview shows the SVG tag and content with basic syntax highlighting
```

### Interactive Search

```bash
# Start interactive search mode
lucide-cli search
```

This opens an interactive search interface where you can:

- Search for icons in real-time
- Preview selected icons
- Download icons directly from the search results

### Download All Icons

```bash
# Download all available icons (1000+ icons!)
lucide-cli all --output ./all-lucide-icons
```

## API Reference

### `LucideClient`

#### Methods

- `new() -> Self` - Create a new client with default settings
- `with_user_agent(user_agent: String) -> Self` - Set custom user agent
- `list_icons() -> Result<Vec<IconInfo>, LucideError>` - Get all available icons
- `search_icons(pattern: &str) -> Result<Vec<IconInfo>, LucideError>` - Search icons by name
- `download_icon(icon_name: &str, output_path: &Path) -> Result<String, LucideError>` - Download single icon
- `download_icons(icon_names: &[&str], output_dir: &Path) -> Result<HashMap<String, Result<String, LucideError>>, LucideError>` - Download multiple icons
- `download_all_icons(output_dir: &Path) -> Result<(usize, Vec<String>), LucideError>` - Download all icons
- `get_icon_content(icon_name: &str) -> Result<String, LucideError>` - Get SVG content as string

### `IconInfo`

```rust
pub struct IconInfo {
    pub name: String,        // e.g., "heart.svg"
    pub download_url: String, // GitHub raw URL
    pub size: u64,           // File size in bytes
}
```

### Error Handling

The library uses custom error types:

```rust
pub enum LucideError {
    Network(ureq::Error),           // Network/HTTP errors
    Json(serde_json::Error),        // JSON parsing errors
    Io(std::io::Error),            // File I/O errors
    IconNotFound(String),          // Icon doesn't exist
}
```

## CLI Commands Reference

| Command    | Description                        | Options                |
| ---------- | ---------------------------------- | ---------------------- |
| `list`     | List available icons               | `--search`, `--limit`  |
| `select`   | Interactive multi-select interface | `--output`, `--search` |
| `download` | Download specific icons by name    | `--output`             |
| `all`      | Download all available icons       | `--output`             |
| `preview`  | Show SVG content of an icon        | None                   |
| `search`   | Interactive search mode            | None                   |

## Examples

### Library Examples

#### Download Icons for a Web App

```rust
use lucide_svg_fetcher::LucideClient;
use std::path::Path;

fn setup_icons_for_webapp() -> Result<(), Box<dyn std::error::Error>> {
    let client = LucideClient::new();

    // Icons commonly used in web applications
    let webapp_icons = [
        "home.svg", "user.svg", "settings.svg", "search.svg",
        "menu.svg", "x.svg", "chevron-down.svg", "chevron-up.svg",
        "heart.svg", "star.svg", "shopping-cart.svg", "bell.svg"
    ];

    let results = client.download_icons(&webapp_icons, Path::new("public/icons"))?;

    for (name, result) in results {
        match result {
            Ok(_) => println!("✓ {}", name),
            Err(e) => println!("✗ {}: {}", name, e),
        }
    }

    Ok(())
}
```

#### Embed Icons in HTML

```rust
use lucide_svg_fetcher::LucideClient;

fn embed_icon_in_html(icon_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = LucideClient::new();
    let svg_content = client.get_icon_content(icon_name)?;

    // Create HTML with embedded SVG
    let html = format!(
        r#"<div class="icon-container">
            {}
        </div>"#,
        svg_content
    );

    Ok(html)
}
```

#### Build an Icon Processing Pipeline

```rust
use lucide_svg_fetcher::LucideClient;
use std::path::Path;

fn process_icons_for_mobile_app() -> Result<(), Box<dyn std::error::Error>> {
    let client = LucideClient::new();

    // Get navigation-related icons
    let nav_icons = client.search_icons("arrow")?;

    for icon in nav_icons {
        // Download the icon
        let svg_content = client.get_icon_content(&icon.name)?;

        // Process the SVG (e.g., change colors, resize, etc.)
        let processed_svg = svg_content
            .replace(r#"stroke="currentColor""#, r#"stroke="#007AFF""#)
            .replace(r#"width="24""#, r#"width="32""#)
            .replace(r#"height="24""#, r#"height="32""#);

        // Save processed version
        let output_path = Path::new("processed_icons").join(&icon.name);
        std::fs::create_dir_all(output_path.parent().unwrap())?;
        std::fs::write(&output_path, processed_svg)?;

        println!("Processed: {}", icon.name);
    }

    Ok(())
}
```

### CLI Examples

#### Workflow for Designers

```bash
# 1. Explore available icons
lucide-cli list --search interface

# 2. Search interactively with preview
lucide-cli search
# (Search for "button", preview options, download favorites)

# 3. Download a curated set for a project
lucide-cli download \
  button play pause stop \
  volume-1 volume-2 volume-x \
  --output ./media-player-icons

# 4. Preview specific icons before final selection
lucide-cli preview volume-2
```

#### Batch Operations for Development

```bash
# Download all arrow icons for navigation
lucide-cli select --search arrow --output ./navigation-icons

# Get all social media related icons
lucide-cli select --search "twitter|facebook|instagram" --output ./social-icons

# Download comprehensive icon set
lucide-cli download \
  home user settings help info \
  search filter sort edit delete \
  save download upload share \
  --output ./common-icons
```

## Advanced Features

### Custom Icon Processing

You can combine the library with other Rust crates for advanced processing:

```rust
// Example with image processing (requires additional dependencies)
use lucide_svg_fetcher::LucideClient;

fn convert_svg_to_png(icon_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = LucideClient::new();
    let svg_content = client.get_icon_content(icon_name)?;

    // Use resvg or similar crate to convert SVG to PNG
    // This is just a conceptual example
    // let png_data = svg_to_png(&svg_content, 256, 256)?;
    // std::fs::write("icon.png", png_data)?;

    Ok(())
}
```

### Integration with Web Frameworks

#### Actix Web Example

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use lucide_svg_fetcher::LucideClient;

async fn serve_icon(path: web::Path<String>) -> Result<HttpResponse> {
    let client = LucideClient::new();
    let icon_name = format!("{}.svg", path.as_str());

    match client.get_icon_content(&icon_name) {
        Ok(svg_content) => Ok(HttpResponse::Ok()
            .content_type("image/svg+xml")
            .body(svg_content)),
        Err(_) => Ok(HttpResponse::NotFound().body("Icon not found")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/icons/{name}", web::get().to(serve_icon))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Performance Considerations

### Caching

For production applications, consider implementing caching:

```rust
use lucide_svg_fetcher::LucideClient;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct CachedLucideClient {
    client: LucideClient,
    cache: Mutex<HashMap<String, String>>,
}

impl CachedLucideClient {
    pub fn new() -> Self {
        Self {
            client: LucideClient::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_icon_cached(&self, icon_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(content) = cache.get(icon_name) {
            return Ok(content.clone());
        }

        let content = self.client.get_icon_content(icon_name)?;
        cache.insert(icon_name.to_string(), content.clone());

        Ok(content)
    }
}
```

### Batch Downloads

When downloading many icons, use the batch methods for better performance:

```rust
// ✅ Good - uses batch download
let icons = ["heart.svg", "home.svg", "user.svg"];
let results = client.download_icons(&icons, output_dir)?;

// ❌ Avoid - individual downloads are slower
for icon in icons {
    client.download_icon(icon, output_dir.join(icon))?;
}
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Run clippy: `cargo clippy`
6. Format code: `cargo fmt`
7. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Lucide](https://lucide.dev/) for providing the beautiful icon library
- The Rust community for excellent crates like `ureq 3.1`, `clap`, and `dialoguer`

## Changelog

### v0.1.0

- Initial release
- Library API for downloading Lucide SVG icons
- CLI tool with interactive selection and preview
- Search functionality
- Batch download support
- Error handling and progress reporting

