use super::time::Time;
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const TOKEN_LENGTH: usize = 6;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, InputObject)]
#[graphql(input_name = "TokenInput")]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Time>,
}

impl Token {
    fn generate(length: Option<usize>) -> String {
        let length = length.unwrap_or(TOKEN_LENGTH);
        let uuid = &Uuid::new_v4().to_string().replace("-", "");
        let token = base64::encode_config(uuid.to_string(), base64::URL_SAFE_NO_PAD);
        let token_len = token.len().min(length);
        token[..token_len].to_string()
    }

    pub fn new() -> Self {
        Self {
            token: Some(Self::generate(None)),
            expires_at: None,
        }
    }

    pub fn new_with_length(length: usize) -> Self {
        Self {
            token: Some(Self::generate(Some(length))),
            expires_at: None,
        }
    }

    pub fn new_with_expiry(expiry: Time) -> Self {
        Self {
            token: Some(Self::generate(None)),
            expires_at: Some(expiry),
        }
    }

    pub fn new_with_expiry_and_length(expiry: Time, length: usize) -> Self {
        Self {
            token: Some(Self::generate(Some(length))),
            expires_at: Some(expiry),
        }
    }

    pub fn validate(&self, other: impl ToString) -> bool {
        if !self.is_valid() {
            return false;
        }
        if let Some(token) = self.token.as_ref() {
            token == &other.to_string()
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.token.is_none() {
            return false;
        }
        if let Some(expires) = self.expires_at {
            expires > Time::now()
        } else {
            true
        }
    }

    pub fn invalidate(&mut self) {
        self.token = None;
        self.expires_at = None;
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        if let Some(token) = self.token.as_ref() {
            token.to_string()
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_length_default() {
        let token = Token::new();
        assert_eq!(super::TOKEN_LENGTH, token.token.unwrap().len());
    }

    #[test]
    fn token_expiry_is_future() {
        let token = Token::new_with_expiry(Time::now() + (60 * 1000));
        assert!(token.expires_at.unwrap().0 > Time::now().0);
    }

    #[test]
    fn token_expiry_is_valid() {
        let token = Token::new_with_expiry(Time::now() + (60 * 1000));
        assert!(token.is_valid());
    }

    #[test]
    fn token_no_expiry_is_valid() {
        let token = Token::new();
        assert!(token.is_valid());
    }
}
