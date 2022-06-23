pub fn to_pig_latin(s: String) -> String {
    let mut result = String::new();

    for word in s.split_whitespace() {
        let first = word.as_bytes()[0];

        if !first.is_ascii_alphabetic() {
            continue;
        }

        match b"aeiou".contains(&first.to_ascii_lowercase()) {
            true => result += &format!("{}-hay ", word.to_lowercase()),
            false => {
                let (first, suffix) = word.split_at(1);
                result += &format!("{}-{}ay ", suffix.to_lowercase(), first.to_lowercase());
            }
        }
    }

    return result.trim().to_string();
}
