pub fn parse(input: &str) -> Result<Vec<i128>, String> {
    input
        .split(",")
        .filter(|s| s.len() > 0)
        .map(|s| match s.parse::<i128>() {
            Ok(i) => Ok(i),
            Err(e) => Err(format!("Invalid integer '{}' ({:?})", s, e)),
        })
        .collect()
}
