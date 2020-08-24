#![deny(elided_lifetimes_in_paths)]

use std::{collections::HashMap, error::Error, fmt, iter::Peekable, str::Chars};

#[derive(PartialEq, Debug, Clone)]
pub enum SymbolType {
    Dot,
    DotDot,
    Exclamation,
    Equals,
    NotEquals,
    Less,
    LessEquals,
    ShiftLeft,
    InetContainedByOrEquals,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

fn new_position() -> Position {
    Position { line: 1, column: 1 }
}

impl Position {
    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

#[test]
fn test_new_line() {
    let mut pos = Position {
        line: 12,
        column: 42,
    };
    pos.new_line();
    assert_eq!(pos.line, 13);
    assert_eq!(pos.column, 1);
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub symbol_type: SymbolType,
    pub position: Position,
    pub value: String,
}

struct ScanNode {
    pub symbol_type: Option<SymbolType>,
    pub children: ScanTree,
}

impl ScanNode {
    pub fn new() -> Self {
        Self {
            symbol_type: None,
            children: ScanTree::new(),
        }
    }
}

struct ScanTree {
    map: HashMap<char, ScanNode>,
}

/*
#[derive(Debug)]
struct InvalidSymbol {}
impl fmt::Display for InvalidSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { write!(f, "invalid symbol") }
}
impl Error for InvalidSymbol {}

#[derive(PartialEq, Debug)]
pub enum ScanError {
    InvalidSymbol,
}
*/

impl ScanTree {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn append(&mut self, s: String, symbol_type: SymbolType) -> &mut Self {
        let ch = match s.chars().next() {
            Some(it) => it,
            _ => return self,
        };
        let node = self.map.entry(ch).or_insert_with(ScanNode::new);
        if s.chars().count() == 1 {
            node.symbol_type = Some(symbol_type);
        } else {
            node.children
                .append(s.chars().skip(1).collect(), symbol_type);
        }

        self
    }

    pub fn longest_match(&self, s: String) -> Option<SymbolType> {
        let ch = match s.chars().next() {
            Some(it) => it,
            _ => return None,
        };

        let node = match self.map.get(&ch) {
            Some(it) => it,
            _ => return None,
        };

        node.children
            .longest_match(s.chars().skip(1).collect()) // TODO: derecursive this
            .or_else(|| node.symbol_type.clone())

        /*match s.chars().nth(0) {
            Some(ch) => match self.map.get(&ch) {
                Some(node) => match node.children.longest_match(s.chars().skip(1).collect()) { // TODO: derecursive this
                    None => node.symbol_type.clone(),
                    it => it,
                },
                _ => None,
            }
            _ => None,
        }*/
    }
}

#[cfg(test)]
mod scan_test {
    use super::{ScanTree, SymbolType};

    #[test]
    fn empty_scan() {
        let scan = ScanTree::new();
        assert_eq!(None, scan.longest_match("abcd".to_owned()));
    }

    #[test]
    fn simple_scan() {
        let mut scan = ScanTree::new();
        scan.append("=".to_owned(), SymbolType::Equals);
        assert_eq!(Some(SymbolType::Equals), scan.longest_match("=".to_owned()));
        assert_eq!(None, scan.longest_match("abcd".to_owned()));
    }

    #[test]
    fn full_scan() {
        let mut scan = ScanTree::new();
        scan.append("=".to_owned(), SymbolType::Equals)
            .append("<".to_owned(), SymbolType::Less)
            .append("<=".to_owned(), SymbolType::LessEquals)
            .append("<<".to_owned(), SymbolType::ShiftLeft)
            .append("<<=".to_owned(), SymbolType::InetContainedByOrEquals);
        assert_eq!(None, scan.longest_match("abcd".to_owned()));
        assert_eq!(Some(SymbolType::Less), scan.longest_match("<".to_owned()));
        assert_eq!(
            Some(SymbolType::LessEquals),
            scan.longest_match("<=".to_owned())
        );
        assert_eq!(
            Some(SymbolType::LessEquals),
            scan.longest_match("<=15".to_owned())
        );
        assert_eq!(
            Some(SymbolType::ShiftLeft),
            scan.longest_match("<<".to_owned())
        );
        assert_eq!(
            Some(SymbolType::InetContainedByOrEquals),
            scan.longest_match("<<=".to_owned())
        );
    }

    #[test]
    fn complex_scan() {
        let mut scan = ScanTree::new();
        scan.append("a".to_owned(), SymbolType::Less)
            .append("abc".to_owned(), SymbolType::InetContainedByOrEquals);
        assert_eq!(Some(SymbolType::Less), scan.longest_match("a".to_owned()));
        assert_eq!(Some(SymbolType::Less), scan.longest_match("ab2".to_owned()));
        assert_eq!(
            Some(SymbolType::InetContainedByOrEquals),
            scan.longest_match("abczzz".to_owned())
        );
    }
}

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    // index: usize,
    position: Position,
}

#[derive(Debug, Clone)]
pub struct UnexpectedCharacter {}

impl fmt::Display for UnexpectedCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown token")
    }
}

impl Error for UnexpectedCharacter {}

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacter,
}

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(s: &str) -> Scanner {
        Scanner {
            chars: s.chars().peekable(),
            // index: 0,
            position: new_position(),
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn from_str(s: &str) -> Scanner {
        Scanner {
            chars: s.chars().peekable(),
            // index: 0,
            position: new_position(),
        }
    }

    pub fn with_string(&mut self, s: &'a str) -> &mut Self {
        self.chars = s.chars().peekable();
        // self.index = 0;
        self.position = new_position();
        self
    }

    pub fn skip_whitespaces(&mut self) -> Result<(), ScannerError> {
        loop {
            match self.chars.peek() {
                None => break,
                Some(c) => {
                    if !c.is_whitespace() {
                        break;
                    }
                }
            }
            self.chars.next();
        }
        Ok(())
    }

    pub fn scan(&mut self) -> Result<Option<Token>, ScannerError> {
        // let start = self.index;
        loop {
            match self.chars.peek() {
                Some(&ch) => {
                    return Ok(Some(match ch {
                        ' ' | '\t' => {
                            // self.index += 1;
                            self.position.column += 1;
                            continue;
                        } // just ignore spaces and tabs
                        '.' => match self.chars.nth(1) {
                            Some(ch2) if ch2 == '.' => {
                                let token = Token {
                                    value: vec![ch, ch2].into_iter().collect(),
                                    symbol_type: SymbolType::DotDot,
                                    position: self.position.clone(),
                                };
                                // self.index += 2;
                                self.position.column += 2;
                                token
                            }
                            _ => {
                                let token = Token {
                                    value: vec![ch].into_iter().collect(),
                                    symbol_type: SymbolType::Dot,
                                    position: self.position.clone(),
                                };
                                // self.index += 1;
                                self.position.column += 1;
                                token
                            }
                        },
                        '!' => match self.chars.nth(1) {
                            Some(ch2) if ch2 == '=' => {
                                let token = Token {
                                    value: vec![ch, ch2].into_iter().collect(),
                                    symbol_type: SymbolType::NotEquals,
                                    position: self.position.clone(),
                                };
                                // self.index += ch.len_utf8() + ch2.len_utf8();
                                self.position.column += 2;
                                token
                            }
                            _ => {
                                let token = Token {
                                    value: vec![ch].into_iter().collect(),
                                    symbol_type: SymbolType::Exclamation,
                                    position: self.position.clone(),
                                };
                                // self.index += 1;
                                self.position.column += 1;
                                token
                            }
                        },
                        '<' => match self.chars.nth(1) {
                            Some(ch2) if ch2 == '>' => {
                                let token = Token {
                                    value: vec![ch, ch2].into_iter().collect(),
                                    symbol_type: SymbolType::NotEquals,
                                    position: self.position.clone(),
                                };
                                // self.index += ch.len_utf8() + ch2.len_utf8();
                                self.position.column += 2;
                                token
                            }
                            Some(ch2) if ch2 == '=' => {
                                let token = Token {
                                    value: vec![ch, ch2].into_iter().collect(),
                                    symbol_type: SymbolType::LessEquals,
                                    position: self.position.clone(),
                                };
                                // self.index += ch.len_utf8() + ch2.len_utf8();
                                self.position.column += 2;
                                token
                            }
                            Some(ch2) if ch2 == '<' => {
                                let token = Token {
                                    value: vec![ch, ch2].into_iter().collect(),
                                    symbol_type: SymbolType::ShiftLeft,
                                    position: self.position.clone(),
                                };
                                // self.index += ch.len_utf8() + ch2.len_utf8();
                                self.position.column += 2;
                                token
                            }
                            _ => {
                                let token = Token {
                                    value: vec![ch].into_iter().collect(),
                                    symbol_type: SymbolType::Less,
                                    position: self.position.clone(),
                                };
                                // self.index += 1;
                                self.position.column += 1;
                                token
                            }
                        },
                        _ => {
                            return Err(ScannerError::UnexpectedCharacter);
                        }
                    }));
                }
                None => {
                    return Ok(None);
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Position, Scanner, SymbolType, Token};
    struct Case<'a> {
        s: &'a str,
        expected: Vec<Token>,
    }

    impl<'a> Case<'a> {
        pub fn run(&self) {
            let mut scanner = Scanner::from(self.s);
            let mut result = vec![];
            while let Some(token) = scanner.scan().unwrap() {
                result.push(token);
            }
            assert_eq!(self.expected, result);
        }
    }

    // #[test]
    fn test_scanner() {
        vec![
            Case {
                s: "",
                expected: vec![],
            },
            Case {
                s: " ",
                expected: vec![],
            },
            Case {
                s: ".",
                expected: vec![Token {
                    symbol_type: SymbolType::Dot,
                    position: Position { line: 1, column: 1 },
                    value: ".".to_owned(),
                }],
            },
            Case {
                s: "..",
                expected: vec![Token {
                    symbol_type: SymbolType::DotDot,
                    position: Position { line: 1, column: 1 },
                    value: "..".to_owned(),
                }],
            },
            Case {
                s: "!",
                expected: vec![Token {
                    symbol_type: SymbolType::Exclamation,
                    position: Position { line: 1, column: 1 },
                    value: "!".to_owned(),
                }],
            },
            Case {
                s: "!=",
                expected: vec![Token {
                    symbol_type: SymbolType::NotEquals,
                    position: Position { line: 1, column: 1 },
                    value: "!=".to_owned(),
                }],
            },
            Case {
                s: "<",
                expected: vec![Token {
                    symbol_type: SymbolType::Less,
                    position: Position { line: 1, column: 1 },
                    value: "<".to_owned(),
                }],
            },
            Case {
                s: "<>",
                expected: vec![Token {
                    symbol_type: SymbolType::NotEquals,
                    position: Position { line: 1, column: 1 },
                    value: "<>".to_owned(),
                }],
            },
            Case {
                s: "<=",
                expected: vec![Token {
                    symbol_type: SymbolType::LessEquals,
                    position: Position { line: 1, column: 1 },
                    value: "<=".to_owned(),
                }],
            },
            Case {
                s: "<<",
                expected: vec![Token {
                    symbol_type: SymbolType::ShiftLeft,
                    position: Position { line: 1, column: 1 },
                    value: "<<".to_owned(),
                }],
            },
        ]
        .iter()
        .for_each(|case| case.run());
    }
}
