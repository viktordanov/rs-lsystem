use std::fmt::Display;
use crate::LSystemError;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Token {
    name: String,
    param: u8,
}

impl Token {

    pub fn new<T: Into<String>>(name: T) -> Result<Self, LSystemError> {
        let name = name.into();

        if name.contains(' ') {
            Err(LSystemError::InvalidToken(name))
        } else {
            let param = parse_param(&name);
            Ok(Self { name, param })
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn param(&self) -> u8 {
        self.param
    }
}


impl Into<Token> for &str {
    fn into(self) -> Token {
        Token::new(self).unwrap()
    }
}

#[derive(Debug, Copy, Eq, Hash, PartialEq)]
pub struct TokenId(pub u8);

impl TokenId {
    pub fn new(mut value: u8, has_param: bool) -> Self {
        assert!(value < 128, "TokenId value must be less than 128; got {}", value);

        let mut param = 0b0000_0000;
        if has_param {
            param = 0b1000_0000;
        }

        value |= param;

        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0 & 0b0111_1111
    }

    pub fn has_param(&self) -> bool {
        self.0 & 0b1000_0000 == 0b1000_0000
    }
}

impl Display for TokenId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value(), if self.has_param() { "p" } else { "" })
    }
}

impl Clone for TokenId {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl From<u8> for TokenId {
    fn from(id: u8) -> Self {
        Self(id)
    }
}


/**
 * Returns the parameter number of a token, if it has one.
 */
fn parse_param(name: &str) -> u8 {
    let mut chars = name.chars().rev();
    let mut param = 0;
    let mut multiplier = 1;

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            param += c.to_digit(10).unwrap() * multiplier;
            multiplier *= 10;
        } else {
            break;
        }
    }

    if param > u8::MAX as u32 {
        return 255;
    }
    param as u8
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name())
    }
}
