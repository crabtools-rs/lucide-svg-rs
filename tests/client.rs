use lucide_svg_rs::{LucideClient, ICONS_TAR};

#[test]
fn test_get_icon_content_returns_svg() {
    let client = LucideClient::new(ICONS_TAR).unwrap();
    let svg = client.get_icon_content("activity").unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_get_icon_content_missing_icon_errors() {
    let client = LucideClient::new(ICONS_TAR).unwrap();
    let result = client.get_icon_content("does-not-exist");
    assert!(result.is_err());
}
