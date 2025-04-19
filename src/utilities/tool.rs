pub fn source_path(capture_file: String) -> String {
    const TEST_DIR: &str = "/Users/tymalik/Docs/Git/markdown/";
    const TEST_PREFIX: &str = "_";
    const TEST_EXT: &str = ".md";
    return format!("{}{}{}{}", TEST_DIR, TEST_PREFIX, capture_file, TEST_EXT);
}
