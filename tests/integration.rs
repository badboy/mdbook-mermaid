use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

fn test_book_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("test-book")
}

fn build_test_book() -> PathBuf {
    let book_dir = test_book_path();
    let output_dir = book_dir.join("book");

    // Clean previous build
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("Failed to clean output directory");
    }

    // Build the book
    Command::new("mdbook")
        .arg("build")
        .current_dir(&book_dir)
        .env(
            "PATH",
            format!(
                "{}:{}",
                env!("CARGO_BIN_EXE_mdbook-mermaid-ssr")
                    .split('/')
                    .collect::<Vec<_>>()
                    .into_iter()
                    .take_while(|&s| s != "target")
                    .collect::<Vec<_>>()
                    .join("/")
                    + "/target/debug",
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .assert()
        .success();

    output_dir
}

#[test]
fn test_book_builds_successfully() {
    let output_dir = build_test_book();

    // Verify output files exist
    assert!(
        output_dir.join("index.html").exists(),
        "index.html should exist"
    );
    assert!(
        output_dir.join("chapter_with_mermaid.html").exists(),
        "chapter_with_mermaid.html should exist"
    );
    assert!(
        output_dir.join("chapter_without_mermaid.html").exists(),
        "chapter_without_mermaid.html should exist"
    );
}

#[test]
fn test_chapter_with_mermaid_content() {
    let output_dir = build_test_book();
    let content = fs::read_to_string(output_dir.join("chapter_with_mermaid.html"))
        .expect("Failed to read chapter_with_mermaid.html");

    // Should contain SVG
    assert!(content.contains("<svg"), "Should contain SVG elements");

    // Should NOT contain mermaid code blocks
    assert!(
        !content.contains("```mermaid"),
        "Should not contain mermaid code blocks"
    );

    // Snapshot the content
    insta::assert_snapshot!("chapter_with_mermaid_html", content);
}

#[test]
fn test_chapter_without_mermaid_content() {
    let output_dir = build_test_book();
    let content = fs::read_to_string(output_dir.join("chapter_without_mermaid.html"))
        .expect("Failed to read chapter_without_mermaid.html");

    // Should preserve other code blocks
    assert!(content.contains("rust"), "Should preserve rust code blocks");
    assert!(
        content.contains("python"),
        "Should preserve python code blocks"
    );

    // Should NOT contain SVG (no mermaid diagrams)
    assert!(!content.contains("<svg"), "Should not contain SVG elements");

    // Snapshot the content
    insta::assert_snapshot!("chapter_without_mermaid_html", content);
}
