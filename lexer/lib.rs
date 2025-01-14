use crate::token::{TokenKind, lookup_identifier, Token, Span};

pub mod token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            } else {
                panic!("read out of range")
            }
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                ch
            } else {
                panic!("read out of range")
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        // println!("self ch {}, position {} read_position {}", self.ch, self.position, self.read_position);
        self.skip_whitespace();
        let t = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::EQ
                } else {
                    TokenKind::ASSIGN
                }
            }
            ';' => TokenKind::SEMICOLON,
            '(' => TokenKind::LPAREN,
            ')' => TokenKind::RPAREN,
            ',' => TokenKind::COMMA,
            '+' => TokenKind::PLUS,
            '-' => TokenKind::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::NotEq
                } else {
                    TokenKind::BANG
                }
            }
            '*' => TokenKind::ASTERISK,
            '/' => TokenKind::SLASH,
            '<' => TokenKind::LT,
            '>' => TokenKind::GT,
            '{' => TokenKind::LBRACE,
            '}' => TokenKind::RBRACE,
            '[' => TokenKind::LBRACKET,
            ':' => TokenKind::COLON,
            ']' => TokenKind::RBRACKET,
            '\u{0}' => TokenKind::EOF,
            '"' => {
                let (start, end, string) = self.read_string();
                return Token { span: Span {start, end},  kind: TokenKind::STRING(string) };
            },
            _ => {
                if is_letter(self.ch) {
                    let (start, end, identifier) = self.read_identifier();
                    return Token { span: Span {start, end}, kind: lookup_identifier(&identifier) };
                } else if is_digit(self.ch) {
                    let (start, end, num) = self.read_number();
                    return Token { span: Span {start, end}, kind: TokenKind::INT(num) };
                } else {
                    TokenKind::ILLEGAL
                }
            }
        };

        self.read_char();
        return Token { span: Span {start: self.position - 1, end: self.read_position - 1, }, kind: t };
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> (usize, usize, String) {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let x = self.input[pos..self.position].to_string();
        return (pos, self.position, x)
    }

    fn read_number(&mut self) -> (usize, usize, i64) {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        let x = self.input[pos..self.position].parse().unwrap();

        return (pos, self.position, x)
    }

    fn read_string(&mut self) -> (usize, usize, String) {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\u{0}' {
                break
            }
        }
        
        let x = self.input[pos..self.position].to_string();

        // consume the end "
        if self.ch == '"'{
            self.read_char();
        }
        return (pos - 1, self.position, x)
    }
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[cfg(test)]
mod tests {
    use crate::Lexer;
    use crate::token::TokenKind;
    use super::*;
    use insta::assert_debug_snapshot;

    fn test_token_set(l: &mut Lexer) -> Vec<Token> {
        let mut token_vs: Vec<Token> = vec![];
        loop {
            let t = l.next_token();
            if t.kind == TokenKind::EOF {
                token_vs.push(t);
                break;
            } else {
                token_vs.push(t);
            }
        }
        token_vs
    }

    #[test]
    fn test_lexer_simple() {
        let mut l = Lexer::new("=+(){},:;");
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_let() {
        let mut l = Lexer::new("let x=5");
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_let_with_space() {
        let mut l = Lexer::new("let x = 5");
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_string() {
        let mut l = Lexer::new(r#""a""#);
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_array() {
        let mut l = Lexer::new("[3]");
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_hash() {
        let mut l = Lexer::new(r#"{"one": 1, "two": 2, "three": 3}"#);
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_bool() {
        let mut l = Lexer::new("let y=true");
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }

    #[test]
    fn test_lexer_complex() {
        let mut l = Lexer::new("let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;");
        let token_vs = test_token_set(&mut l);

        assert_debug_snapshot!(token_vs)
    }
}
