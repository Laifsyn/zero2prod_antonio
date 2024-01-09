use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(String);
impl SubscriberName {
    unsafe fn parse(s: &str) -> SubscriberName {
        Self(s.to_string())
    }
    fn is_valid(s: &str) -> bool {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
    }
}

impl TryFrom<&str> for SubscriberName {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match SubscriberName::is_valid(value) {
            true => Ok(unsafe { SubscriberName::parse(value) }),
            false => Err(format!("{:?} is not a valid subscriber name", value)),
        }
    }
}
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
