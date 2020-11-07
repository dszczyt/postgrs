use regex;
use std::iter::Iterator;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Id,
    Sid,
    Open,
    XClose,
    XCreate,
    ObjId,
    XBootstrap,
    XSharedRelation,
}

struct Location {
    pub line: usize,
    pub col: usize,
}
impl Location {
    pub fn new() -> Self {
        Self { line: 1, col: 1 }
    }
}

struct Token {
    pub toktype: TokenType,
    pub start: usize,
    pub end: usize,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
struct TokenMatch {
    pub toktype: TokenType,
    pub len: usize,
}

trait Matcher {
    fn run(&self, s: &str) -> Option<TokenMatch>;
}

struct RegexMatcher {
    pub toktype: TokenType,
    pub regex: regex::Regex,
}

impl Matcher for RegexMatcher {
    fn run(&self, s: &str) -> Option<TokenMatch> {
        self.regex.find(s).and_then(|mat| {
            Some(TokenMatch {
                toktype: self.toktype.clone(),
                len: mat.range().count(),
            })
        })
        // match self.regex.find(s) {
        //     Some(mat) => Some(TokenMatch{toktype: self.toktype.clone(), len: mat.range().count()}),
        //     None => None
        // }
    }
}

struct ExactMatcher {
    pub toktype: TokenType,
    pub value: String,
}

impl Matcher for ExactMatcher {
    fn run(&self, s: &str) -> Option<TokenMatch> {
        if s.starts_with(&self.value) {
            Some(TokenMatch {
                toktype: self.toktype.clone(),
                len: self.value.chars().count(),
            })
        } else {
            None
        }
        // match s.starts_with(&self.value) {
        //     true => Some(TokenMatch{toktype: self.toktype.clone(), len: self.value.chars().count()}),
        //     false => None
        // }
    }
}

struct CaseInsensitiveExactMatcher {
    pub toktype: TokenType,
    pub value: String,
}

impl Matcher for CaseInsensitiveExactMatcher {
    fn run(&self, s: &str) -> Option<TokenMatch> {
        if s.to_lowercase().starts_with(&self.value) {
            Some(TokenMatch {
                toktype: self.toktype.clone(),
                len: self.value.chars().count(),
            })
        } else {
            None
        }
    }
}

struct Parser {
    matchers: Vec<Box<dyn Matcher>>,
    location: Location,
    s: Option<String>,
}

impl Parser {
    pub fn init() -> Self {
        Self {
            matchers: vec![
                Box::new(RegexMatcher {
                    toktype: TokenType::Id,
                    regex: regex::Regex::new("\\A[-A-Za-z0-9_]+").unwrap(),
                }),
                Box::new(ExactMatcher {
                    toktype: TokenType::Open,
                    value: "open".to_string(),
                }),
                Box::new(ExactMatcher {
                    toktype: TokenType::XClose,
                    value: "close".to_string(),
                }),
            ],
            location: Location::new(),
            s: None,
        }
    }

    fn next_match(&self, s: &str) -> Option<TokenMatch> {
        (&self.matchers)
            .into_iter()
            .filter_map(|matcher| matcher.run(s))
            .max_by_key(|matcher| matcher.len)
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, TokenMatch, TokenType};

    macro_rules! parse_test {
        ( $($name:ident: ($s:expr, $tokenMatch:expr)),* ) => {
            $(
                #[test]
                fn $name() {
                    let parser = Parser::init();
                    assert_eq!(
                        parser.next_match($s),
                        Some($tokenMatch),
                    );
                }
            )*
        };
    }

    parse_test! {
        parse_open: ("open xxx", TokenMatch{
            toktype: TokenType::Open,
            len: 4,
        }),
        parse_close: ("close open", TokenMatch{
            toktype: TokenType::XClose,
            len: 5,
        })
    }
}
