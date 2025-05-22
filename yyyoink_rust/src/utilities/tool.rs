pub fn source_dir() -> String {
    const TEST_DIR: &str = "/Users/tymalik/Docs/Git/markdown/";
    return format!("{}", TEST_DIR);
}

pub fn source_path(capture_file: String) -> String {
    let source_dir = source_dir();
    let test_dir: &str = source_dir.as_ref();
    return format!("{}{}", test_dir, capture_file);
}
