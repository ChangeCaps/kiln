use std::{iter::Peekable, str::Chars};

use crate::{Error, Result, SourceId, Spanned, Token};

struct Lexer<'a> {
    index: usize,
    chars: Peekable<Chars<'a>>,
    source: SourceId,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, source_id: SourceId) -> Self {
        Self {
            index: 0,
            chars: source.chars().peekable(),
            source: source_id,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.index += ch.len_utf8();

            Some(ch)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, char::is_whitespace) {
            self.next();
        }
    }

    pub fn parse_token(&mut self) -> Result<Spanned<Token>> {
        let ch = self.peek().ok_or_else(|| Error::new());
        todo!()
    }
}
