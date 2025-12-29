use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;

static BUILD_ONCE: Once = Once::new();

fn test_book_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("test-book")
}

fn output_dir() -> PathBuf {
    test_book_dir().join("book")
}

fn ensure_built() {
    BUILD_ONCE.call_once(|| {
        let book_dir = test_book_dir();
        let output = output_dir();

        // Clean previous build to ensure we test current code
        if output.exists() {
            fs::remove_dir_all(&output).expect("Failed to clean output directory");
        }

        // Build the book using mdbook
        let status = Command::new("mdbook")
            .arg("build")
            .current_dir(&book_dir)
            .status()
            .expect("Failed to run mdbook build");

        assert!(status.success(), "mdbook build failed");
    });
}

#[test]
fn test_book_builds() {
    ensure_built();

    let output = output_dir();
    assert!(output.exists(), "Output directory should exist");
    assert!(
        output.join("index.html").exists(),
        "index.html should exist"
    );
}

#[test]
fn test_chapter_with_mermaid() {
    ensure_built();

    let content = fs::read_to_string(output_dir().join("chapter_with_mermaid.html"))
        .expect("Failed to read chapter_with_mermaid.html");

    // Should contain mermaid-generated SVG elements (check for SVG with typical mermaid structure)
    assert!(
        content.contains("<svg") && content.contains("flowchart"),
        "Chapter with mermaid should contain mermaid-generated SVG"
    );

    // Should NOT contain mermaid code blocks
    assert!(
        !content.contains("```mermaid"),
        "Should not contain raw mermaid blocks"
    );

    // Snapshot the HTML content
    insta::assert_snapshot!("chapter_with_mermaid_html", content);
}

#[test]
fn test_chapter_without_mermaid() {
    ensure_built();

    let content = fs::read_to_string(output_dir().join("chapter_without_mermaid.html"))
        .expect("Failed to read chapter_without_mermaid.html");

    // Should NOT contain mermaid diagrams
    // (Note: mdBook may include SVG icons and the word "mermaid" in text,
    // so we check for absence of mermaid diagram markers)
    assert!(
        !content.contains("flowchart") && !content.contains("sequenceDiagram"),
        "Chapter without mermaid should not contain mermaid diagram content"
    );

    // Should preserve code blocks
    assert!(content.contains("rust"), "Should preserve rust code blocks");

    // Snapshot the HTML content
    insta::assert_snapshot!("chapter_without_mermaid_html", content);
}
