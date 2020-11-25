use crate::token::{Token, lookup_identifier};

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
        self.skip_whitespace();
        let t = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            },
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::BANG
                }
            },
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            '{' => Token::LPAREN,
            '}' => Token::RPAREN,
           '\u{0}' => Token::EOF,
            _ => {
                if is_letter(self.ch) {
                    // has to return, I am not sure rust goes this way :(
                    return lookup_identifier(&self.read_identifier())
                } else if is_digit(self.ch) {
                    return Token::INT(self.read_number())
                } else {
                    Token::ILLEGAL
                }
            }
        };

        self.read_char();
        return t;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let x = self.input[pos..self.position].to_string();
        x
    }

    fn read_number(&mut self) -> i64 {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[pos..self.position].parse().unwrap()
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
    use crate::token::Token;
    use super::*;
    use insta::assert_debug_snapshot;

    fn test_token_set(l: &mut Lexer) -> Vec<Token> {
        let mut token_vs: Vec<Token> = vec![];
        loop {
            let t = l.next_token();
            if t == Token::EOF {
                token_vs.push(t);
                break
            } else {
                token_vs.push(t);
            }
        }
        token_vs
    }

    #[test]
    fn test_lexer_simple() {
        let mut l = Lexer::new("=+(){},;");
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
        let mut token_vs: Vec<Token> = vec![];
        loop {
            let t = l.next_token();
            if t == Token::EOF {
                token_vs.push(t);
                break
            } else {
                token_vs.push(t);
            }
        }

        assert_debug_snapshot!(token_vs)
    }
}
