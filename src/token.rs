#![allow(non_camel_case_types)]

use std::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Token {
    AnchorBegin,
    AnchorEnd,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    ParenthesesLeft,
    ParenthesesRight,
    QuestionMark,
    Asterisk,
    Plus,
    AnyChar,
    Value(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::AnchorBegin => write!(f, "^"),
            Token::AnchorEnd => write!(f, "$"),
            Token::BraceLeft => write!(f, "{{"),
            Token::BraceRight => write!(f, "}}"),
            Token::BracketLeft => write!(f, "["),
            Token::BracketRight => write!(f, "]"),
            Token::ParenthesesLeft => write!(f, "("),
            Token::ParenthesesRight => write!(f, ")"),
            Token::QuestionMark => write!(f, "?"),
            Token::Asterisk => write!(f, "*"),
            Token::Plus => write!(f, "+"),
            Token::AnyChar => write!(f, "."),
            Token::Value(c) => write!(f, "Val({})", c),
        }
    }
}

pub fn lookup_token(id: char) -> Token {
    match id {
        '^' => Token::AnchorBegin,
        '$' => Token::AnchorEnd,
        '{' => Token::BraceLeft,
        '}' => Token::BraceRight,
        '[' => Token::BracketLeft,
        ']' => Token::BracketRight,
        '(' => Token::ParenthesesLeft,
        ')' => Token::ParenthesesRight,
        '?' => Token::QuestionMark,
        '*' => Token::Asterisk,
        '+' => Token::Plus,
        '.' => Token::AnyChar,
        _ => Token::Value(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lookup_token() {
        let src: Vec<Token> = "^{) *+c$".chars().map(|c| lookup_token(c)).collect();
        let dest: Vec<Token> = vec![
            Token::AnchorBegin,
            Token::BraceLeft,
            Token::ParenthesesRight,
            Token::Value(' '),
            Token::Asterisk,
            Token::Plus,
            Token::Value('c'),
            Token::AnchorEnd,
        ];
        assert_eq!(src.len(), dest.len());
        assert_eq!(src, dest);
    }
}
