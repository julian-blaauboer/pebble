use std::iter::Peekable;

// Lexical tokens for parsing
#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    Equals,
    Let,
    Number(f64),
    Identifier(String),
}

// The default tokenizer takes a char iterator as input and outputs tokens
pub struct Tokenizer<T>
where
    T: Iterator<Item = char>,
{
    pub input: Peekable<T>,
}

impl<T> Tokenizer<T>
where
    T: Iterator<Item = char>,
{
    // Helper function for parsing numbers
    fn parse_number(&mut self, first: char) -> Option<f64> {
        let mut text = first.to_string();
        while let Some('0'..='9') = self.input.peek() {
            text.push(self.input.next().unwrap());
        }
        if let Some(x) = self.input.peek() {
            if *x == '.' {
                text.push(self.input.next().unwrap());
                while let Some('0'..='9') = self.input.peek() {
                    text.push(self.input.next().unwrap());
                }
            }
        }
        Some(
            text.parse::<f64>()
                .expect("logic error, number should be able to be parsed."),
        )
    }

    // Helper function for parsing identifiers
    fn parse_identifier(&mut self, first: char) -> Option<String> {
        let mut text = first.to_string();
        while let Some('a'..='z') = self.input.peek() {
            text.push(self.input.next().unwrap());
        }
        Some(text)
    }

    // Helper function for parsing reserved words
    fn parse_reserved(word: String) -> Token {
        match &word[..] {
            "let" => Token::Let,
            _ => Token::Identifier(word),
        }
    }
}

impl<T> Iterator for Tokenizer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        use Token::*;
        Some(match self.input.next()? {
            ' ' | '\r' | '\n' | '\t' => self.next()?,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '(' => LParen,
            ')' => RParen,
            ',' => Comma,
            '=' => Equals,
            x @ '0'..='9' => Number(self.parse_number(x)?),
            x @ 'a'..='z' => Tokenizer::<T>::parse_reserved(self.parse_identifier(x)?),
            _ => return None,
        })
    }
}
