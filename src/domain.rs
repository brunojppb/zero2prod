use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(input: String) -> Result<Self, String> {
        let is_empty_or_whitespace = input.trim().is_empty();

        let is_too_long = input.graphemes(true).count() > 255;

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbidden_chars = input.chars().any(|c| forbidden_chars.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not a valid subscriber name", input))
        } else {
            Ok(Self(input))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
