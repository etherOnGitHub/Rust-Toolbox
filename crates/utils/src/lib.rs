pub fn slugify(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "-")
}
