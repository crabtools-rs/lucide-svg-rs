use lucide_svg_rs::LucideClient;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LucideClient::new();

    // Example 1: List all icons
    println!("=== Listing all icons ===");
    let icons = client.list_icons()?;
    println!("Found {} icons", icons.len());

    // Show first 5 icons
    for icon in icons.iter().take(5) {
        println!("- {} ({} bytes)", icon.name, icon.size);
    }

    // Example 2: Search for specific icons
    println!("\n=== Searching for heart icons ===");
    let heart_icons = client.search_icons("heart")?;
    for icon in &heart_icons {
        println!("- {}", icon.name);
    }

    // Example 3: Download specific icons
    println!("\n=== Downloading selected icons ===");
    let icons_to_download = ["heart.svg", "home.svg", "user.svg"];
    let output_dir = Path::new("downloaded_icons");

    let results = client.download_icons(&icons_to_download, output_dir)?;

    for (name, result) in results {
        match result {
            Ok(_) => println!("✓ Downloaded: {name}"),
            Err(e) => println!("✗ Failed to download {name}: {e}"),
        }
    }

    // Example 4: Get icon content without saving
    println!("\n=== Getting icon content ===");
    match client.get_icon_content("heart.svg") {
        Ok(content) => {
            println!("Heart icon content length: {} bytes", content.len());
            // You can now use the SVG content in your application
            // For example, embed it in HTML, process it, etc.
        }
        Err(e) => println!("Failed to get content: {e}"),
    }

    // Example 5: Download a single icon
    println!("\n=== Downloading single icon ===");
    let single_output = Path::new("single_icon/star.svg");
    match client.download_icon("star.svg", single_output) {
        Ok(_) => println!("✓ Downloaded star icon to {single_output:?}"),
        Err(e) => println!("✗ Failed to download star icon: {e}"),
    }

    // Example 6: Custom user agent
    println!("\n=== Using custom client ===");
    let custom_client = LucideClient::new().with_user_agent("my-app/1.0".to_string());

    let custom_icons = custom_client.list_icons()?;
    println!("Custom client found {} icons", custom_icons.len());

    Ok(())
}
