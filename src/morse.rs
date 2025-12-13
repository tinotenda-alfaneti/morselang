pub fn morse_to_keyword(code: &str) -> Option<&'static str> {
    match code {
        ".. ..-." => Some("IF"),
        ". .-.. ... ." => Some("ELSE"),
        ".-- .... .. .-.. ." => Some("WHILE"),
        "..-. --- .-." => Some("FOR"),
        "-.. ---" => Some("DO"),
        ".-. -.." => Some("END"),
        ".--. .-. .. -. -" => Some("PRINT"),
        ".-. . - ..- .-. -." => Some("RETURN"),
        "..-. ..- -. -.-." => Some("FUNC"),
        "... . -" => Some("SET"),
        "- .-. ..- ." => Some("TRUE"),
        "..-. .- .-.. ... ." => Some("FALSE"),
        ".- -. -.." => Some("AND"),
        "--- .-." => Some("OR"),
        "-. --- -" => Some("NOT"),
        _ => None,
    }
}
