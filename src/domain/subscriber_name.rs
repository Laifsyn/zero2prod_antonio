use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub struct SubscriberName(String);
impl SubscriberName {
    const INVALID_CHARACTERS: &'static [char] = &['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    unsafe fn parse_unchecked(s: &str) -> SubscriberName {
        Self(s.to_string())
    }
    fn is_valid(s: &str) -> bool {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = Self::INVALID_CHARACTERS;
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
    }
}

impl TryFrom<&str> for SubscriberName {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match SubscriberName::is_valid(value) {
            true => Ok(unsafe { SubscriberName::parse_unchecked(value) }),
            false => Err(format!("{:?} is not a valid subscriber name", value)),
        }
    }
}

impl TryFrom<String> for SubscriberName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SubscriberName::try_from(value.as_str())
    }
}
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::try_from(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in SubscriberName::INVALID_CHARACTERS {
            let name = name.to_string();
            assert_err!(SubscriberName::try_from(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::try_from(name));
    }
}
