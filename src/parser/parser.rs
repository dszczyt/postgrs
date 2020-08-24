// use std::rc::{Rc, Weak};
use super::identifiers::Identifiers;
use super::keywords::Keyword;
use std::{boxed::Box, ops::Fn};

// trait Matcher {
//     fn match_exact(self, s: &str) -> Result<Token, String>;
// }

// pub type TokenType = dyn Keyword + Identifiers;

// #[derive(Debug)]
// pub enum TokenType {
//     // Keyword(Keyword),
//     /*Space,
//     Semicolon,
//     Coma,

//     Star,

//     Create,
//     */
// }

pub struct AST {}

/*#[derive(Debug)]
pub struct Token {
    pub xtype: TokenType,
    pub lexeme: String,
}

pub struct Lexer {
    pub command: String,
    pos: usize,
}

impl Lexer {
    pub fn next(&mut self) -> Result<Token, String> {
        let matchers: Vec<Box<dyn Fn() -> Result<Token, String>>> = vec![
            // self.match_exact("select", TokenType::Select),
            // self.match_exact("insert", TokenType::Insert),
        ];
        matchers.iter().map(|f| f()).filter(|r| r.is_ok()).nth(0).ok_or("unable to parse").unwrap()
    }

    fn match_exact(&mut self, s: &'static str, t: TokenType) -> Box<dyn Fn() -> Result<Token, String>> {
        Box::new(|| {
            // if self.command.starts_with(s) {
            //     let lexeme = self.command[self.pos..self.pos+s.len()].to_owned();
            //     self.pos += s.len();
            //     return Ok(Token{
            //         xtype: t,
            //         lexeme,
            //     });
            // }
            Err("token not expected".to_owned())
        })
    }
}*/

// pub fn raw_parser(s: String) -> Vec<Token> {
//     vec![]
// }

// pub struct NodeMatcher<T> where T: Matcher {
//     pub matcher: T,
//     pub next: Vec<Self>,
// }

use std::io::Read;

use super::node::NodeTag;
use crate::types::oid::Oid;

pub struct ListCell {
    pub ptr_value: String,
    pub int_value: i64,
    pub oid_value: Oid,
}

pub struct List {
    pub xtype: NodeTag, /* T_List, T_IntList, or T_OidList */
    pub length: usize,
    pub max_length: usize,
    pub elements: ListCell,
    pub initial_elements: Vec<ListCell>,
}

pub enum State {
    Start,
}

#[derive(Debug)]
pub enum TokenType {
    Select,
    Space,
    Identifier,
    ParenOpen,
    ParenClose,
    Comma,
    Dot,
    SemiColon,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub length: usize,
    pub xtype: TokenType,
}

pub fn raw_parser(s: String) -> Vec<Token> {
    let mut start = 0;
    let mut length = 0;
    let mut end = 0;

    let mut tokens: Vec<Token> = vec![];
    for ch in s.chars() {
        length += 1;
        end += ch.len_utf8();

        match &s[start..end] {
            x if x.to_owned().to_uppercase() == "SELECT" => {
                tokens.push(Token {
                    start: start,
                    end: end,
                    length: length,
                    xtype: TokenType::Select,
                });
                start = end;
                length = 0;
            }
            x if x.trim().is_empty() => {
                tokens.push(Token {
                    start: start,
                    end: end,
                    length: length,
                    xtype: TokenType::Space,
                });
                start = end;
                length = 0;
            }
            _ => {}
        }
        println!("{} {}", ch, ch.len_utf8());
    }

    tokens
}