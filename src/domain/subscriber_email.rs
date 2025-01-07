#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(input: String) -> Result<Self, String> {
        if validator::ValidateEmail::validate_email(&input) {
            Ok(Self(input))
        } else {
            Err(format!("{} is not a valida subscriber email", &input))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_err;

    use crate::domain::SubscriberEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn empty_missing_at_symbol_is_rejected() {
        let email = "jamesbond.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@bond.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
