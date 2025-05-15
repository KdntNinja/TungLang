use regex::Regex;

/// Replaces code patterns using a list of phrases and their replacements.
/// Automatically adds `\b` and `\s*` to either side of the phrase for easier expansion.
pub fn preprocess_code(code: &str) -> String {
    let replacements = vec![
        ("capu", "var"),
        ("tung(", "print("),
        ("sahur(", "input("),
        ("tripi(", "int("),
        ("tralalelo(", "quit("),
        ("la_vaca", "if"),
        ("saturno", "elif"),
        ("saturnita", "else"),
        ("bombadillo", "while"),
        ("tralala", "for"),
        ("wa", "fn"),
        // Add more (phrase, replacement) pairs here as needed
    ];
    let mut result = code.to_string();
    for (phrase, replacement) in replacements {
        // Automatically build the regex pattern
        let pattern = format!(r"\b{}\s*", regex::escape(phrase));
        let re = Regex::new(&pattern).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    result
}
