use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SafeString(pub String);

impl SafeString {
    pub fn parse(s: &str) -> Result<SafeString, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("Invalid name! Given: {}", s))
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl AsRef<str> for SafeString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::model::SafeString;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapeme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SafeString::parse(name.as_str()));
    }

    #[test]
    fn a_name_longer_than_256_grapehemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SafeString::parse(name.as_str()));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SafeString::parse(name.as_str()));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SafeString::parse(name.as_str()));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SafeString::parse(name.as_str()));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Muneer Lalji".to_string();
        assert_ok!(SafeString::parse(name.as_str()));
    }
}
