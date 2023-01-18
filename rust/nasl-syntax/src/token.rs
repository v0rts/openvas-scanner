use std::ops::Range;

use crate::ACT;

///! This module defines the TokenTypes as well as Token and extends Cursor with advance_token
use crate::cursor::Cursor;

/// Identifies if a string is quotable or unquotable
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringCategory {
    /// Defines a string as capable of quoting
    ///
    /// Quotable strings will interpret \n\t...
    Quotable, // '..\''
    /// Defines a string as incapable of quoting
    ///
    /// Unquotable strings will use escaped characters as is instead of interpreting them.
    Unquotable, // "..\"
}

/// Identifies if number is base10, base 8, hex or binary
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base {
    /// Base 2: contains 01 is defined by 0b e.g.: `0b010101`
    Binary,
    /// Base 8: contains 0-8 is defined by a starting 0 e.g.: `0123456780`
    Octal,
    /// Base 10: contains 0-9 is the default e.g.: `1234567890`
    Base10,
    /// Base 16: contains 0-9A-F is defined by a starting 0x e.g.: `0x123456789ABCDEF0`
    Hex,
}

impl Base {
    fn verify_binary(peeked: char) -> bool {
        peeked == '0' || peeked == '1'
    }

    fn verify_octal(peeked: char) -> bool {
        ('0'..='7').contains(&peeked)
    }

    fn verify_base10(peeked: char) -> bool {
        ('0'..='9').contains(&peeked)
    }

    fn verify_hex(peeked: char) -> bool {
        ('0'..='9').contains(&peeked)
            || ('A'..='F').contains(&peeked)
            || ('a'..='f').contains(&peeked)
    }
    pub(crate) fn verifier(self) -> impl Fn(char) -> bool {
        match self {
            Self::Binary => Self::verify_binary,
            Self::Octal => Self::verify_octal,
            Self::Base10 => Self::verify_base10,
            Self::Hex => Self::verify_hex,
        }
    }

    /// Returns the radix
    pub fn radix(&self) -> u32 {
        match self {
            Base::Binary => 2,
            Base::Octal => 8,
            Base::Base10 => 10,
            Base::Hex => 16,
        }
    }
}

/// Is used to identify which Category type is unclosed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnclosedCategory {
    /// Is a unclosed String.
    String(StringCategory),
}

macro_rules! make_keyword_matcher {
    ($($matcher:ident : $define:expr),+) => {

impl IdentifierType {
    /// Creates a new keyword based on a string identifier
    pub fn new(keyword: &str) -> Self {
        match keyword {
           $(
           stringify!($matcher) => $define,
           )*
            _ => Self::Undefined(keyword.to_owned())
        }

    }

}

impl ToString for IdentifierType {
    fn to_string(&self) -> String {
        $(
        // special case that is not defined via macro_call
        if let IdentifierType::Undefined(r) = self {
            return r.clone();
        }
        // cannot use match here because define is an expression
        if self == &$define {
            return stringify!($matcher).to_owned();
        }
        )*
        return "".to_owned();
    }
}
    };
}

/// Unless Dynamic those are reserved words that cannot be reused otherwise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentifierType {
    /// function declaration
    Function,
    /// _FCT_ANON_ARGS
    FCTAnonArgs,
    /// TRUE
    True,
    /// FALSE
    False,
    /// for
    For,
    /// foreach
    ForEach,
    /// if
    If,
    /// else
    Else,
    /// while
    While,
    /// repeat
    Repeat,
    /// until
    Until,
    /// local_var
    LocalVar,
    /// global_var
    GlobalVar,
    /// NULL
    Null,
    /// return
    Return,
    /// include
    Include,
    /// Scanning phases; can be set by category in the description block
    ACT(ACT),
    /// exit
    Exit,
    /// Undefined
    Undefined(String),
}

impl From<&Token> for Range<usize> {
    fn from(token: &Token) -> Self {
        let (start, end) = token.position;
        Range { start, end }
    }
}

make_keyword_matcher! {
    function: IdentifierType::Function,
    _FCT_ANON_ARGS: IdentifierType::FCTAnonArgs,
    TRUE: IdentifierType::True,
    FALSE: IdentifierType::False,
    for: IdentifierType::For,
    foreach: IdentifierType::ForEach,
    if: IdentifierType::If,
    else: IdentifierType::Else,
    while: IdentifierType::While,
    repeat: IdentifierType::Repeat,
    until: IdentifierType::Until,
    local_var: IdentifierType::LocalVar,
    global_var: IdentifierType::GlobalVar,
    NULL: IdentifierType::Null,
    return: IdentifierType::Return,
    include: IdentifierType::Include,
    exit: IdentifierType::Exit,
    ACT_ATTACK: IdentifierType::ACT(ACT::Attack),
    ACT_DENIAL: IdentifierType::ACT(ACT::Denial),
    ACT_DESTRUCTIVE_ATTACK: IdentifierType::ACT(ACT::DestructiveAttack),
    ACT_END: IdentifierType::ACT(ACT::End),
    ACT_FLOOD: IdentifierType::ACT(ACT::Flood),
    ACT_GATHER_INFO: IdentifierType::ACT(ACT::GatherInfo),
    ACT_INIT: IdentifierType::ACT(ACT::Init),
    ACT_KILL_HOST: IdentifierType::ACT(ACT::KillHost),
    ACT_MIXED_ATTACK: IdentifierType::ACT(ACT::MixedAttack),
    ACT_SCANNER: IdentifierType::ACT(ACT::Scanner),
    ACT_SETTINGS: IdentifierType::ACT(ACT::Settings)
}

/// Is used to identify a Token
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Category {
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `[`
    LeftBrace,
    /// `]`
    RightBrace,
    /// `{`
    LeftCurlyBracket,
    /// `}`
    RightCurlyBracket,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `%`
    Percent,
    /// `%=`
    PercentEqual,
    /// `;`
    Semicolon,
    /// `:`
    DoublePoint,
    /// `~`
    Tilde,
    /// `^`
    Caret,
    /// `&`
    Ampersand,
    /// `&&`
    AmpersandAmpersand,
    /// `|`
    Pipe,
    /// `||`
    PipePipe,
    /// `!`
    Bang,
    /// `!=`
    BangEqual,
    /// `!~`
    BangTilde,
    /// `=`
    Equal,
    /// `==`
    EqualEqual,
    /// `=~`
    EqualTilde,
    /// `>`
    Greater,
    /// `>>`
    GreaterGreater,
    /// `>=`
    GreaterEqual,
    /// `><`
    GreaterLess,
    /// `<`
    Less,
    /// `<<`
    LessLess,
    /// `<=`
    LessEqual,
    /// `-`
    Minus,
    /// `--`
    MinusMinus,
    /// `-=`
    MinusEqual,
    /// `+`
    Plus,
    /// `+=`
    PlusEqual,
    /// `++`
    PlusPlus,
    /// `/`
    Slash,
    /// `/=`
    SlashEqual,
    /// `*`
    Star,
    /// `**`
    StarStar,
    /// `*=`
    StarEqual,
    /// `>>>`
    GreaterGreaterGreater,
    /// `>>=`
    GreaterGreaterEqual,
    /// `<<=`
    LessLessEqual,
    /// `>!<`
    GreaterBangLess,
    /// `>>>=`
    GreaterGreaterGreaterEqual,
    /// `x` is a special functionality to redo a function call n times.E.g. `send_packet( udp, pcap_active:FALSE ) x 200;`
    X,
    /// A String can be either Quotable (') or Unquotable (") both can be multiline
    String(String),
    /// A Number can be either binary (0b), octal (0), base10 (1-9) or hex (0x)
    Number(i64),
    /// We currently just support 127.0.0.1 notation
    IPv4Address,
    /// Wrongfully identified as IpV4
    IllegalIPv4Address,
    /// An illegal Number e.g. 0b2
    IllegalNumber(Base),
    /// A comment starts with # and should be ignored
    Comment,
    /// Identifier are literals that are not strings and don't start with a number
    Identifier(IdentifierType),
    /// Unclosed token. This can happen on e.g. string literals
    Unclosed(UnclosedCategory),
    /// Number starts with an unidentifiable base
    UnknownBase,
    /// used when the symbol is unknown
    UnknownSymbol,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Contains the TokenType as well as the position in form of Range<usize>
pub struct Token {
    /// The category or kind of a token
    pub category: Category,
    // using a tuple in favor of Range to have the possibility
    // to easily copy tokens rather than clone; to create a range for lookups
    // call range()
    /// The byte position within the given source code
    pub position: (usize, usize),
}

impl Token {
    /// Returns the Category
    pub fn category(&self) -> &Category {
        &self.category
    }

    /// Returns true when an Token is faulty
    ///
    /// A Token is faulty when it is a syntactical error like
    /// - [Category::IllegalIPv4Address]
    /// - [Category::Unclosed]
    /// - [Category::UnknownBase]
    /// - [Category::UnknownSymbol]
    /// - [Category::IllegalNumber]
    pub fn is_faulty(&self) -> bool {
        matches!(
            self.category(),
            Category::IllegalIPv4Address
                | Category::IllegalNumber(_)
                | Category::Unclosed(_)
                | Category::UnknownBase
                | Category::UnknownSymbol
        )
    }
}

/// Tokenizer uses a cursor to create tokens
#[derive(Clone)]
pub struct Tokenizer<'a> {
    // Is used to lookup keywords
    code: &'a str,
    cursor: Cursor<'a>,
}

// Is used to build Some(Token{ ... }) to make the match case within Iterator for Tokenizer easier to read
macro_rules! token {
    ($category:expr, $start:expr, $end:expr) => {
        Some(Token {
            category: $category,
            position: ($start, $end),
        })
    };
}

impl<'a> Tokenizer<'a> {
    /// Creates a new Tokenizer
    pub fn new(code: &'a str) -> Self {
        Tokenizer {
            code,
            cursor: Cursor::new(code),
        }
    }

    /// Returns a reference of a substring within code at given range
    pub fn lookup(&self, range: Range<usize>) -> &'a str {
        &self.code[range]
    }

    // we break out of the macro since > can be parsed to:
    // >>>
    // >>=
    // >>>=
    // >!<
    // most operators don't have triple or tuple variant
    #[inline(always)]
    fn tokenize_greater(&mut self) -> Option<Token> {
        use Category::*;
        let start = self.cursor.len_consumed() - 1;
        let next = self.cursor.peek(0);
        match next {
            '=' => {
                self.cursor.advance();
                token!(GreaterEqual, start, self.cursor.len_consumed())
            }
            '<' => {
                self.cursor.advance();
                token!(GreaterLess, start, self.cursor.len_consumed())
            }
            '>' => {
                self.cursor.advance();
                let next = self.cursor.peek(0);
                match next {
                    '>' => {
                        self.cursor.advance();
                        if self.cursor.peek(0) == '=' {
                            self.cursor.advance();
                            return token!(
                                GreaterGreaterGreaterEqual,
                                start,
                                self.cursor.len_consumed()
                            );
                        }

                        token!(GreaterGreaterGreater, start, self.cursor.len_consumed())
                    }
                    '=' => {
                        self.cursor.advance();
                        token!(GreaterGreaterEqual, start, self.cursor.len_consumed())
                    }
                    _ => token!(GreaterGreater, start, self.cursor.len_consumed()),
                }
            }
            '!' if self.cursor.peek(1) == '<' => {
                self.cursor.advance();
                self.cursor.advance();
                token!(GreaterBangLess, start, self.cursor.len_consumed())
            }
            _ => token!(Greater, start, self.cursor.len_consumed()),
        }
    }

    // we break out of the macro since < can be parsed to:
    // <<=
    // most operators don't have triple or tuple variant
    #[inline(always)]
    fn tokenize_less(&mut self) -> Option<Token> {
        use Category::*;
        let start = self.cursor.len_consumed() - 1;
        let next = self.cursor.peek(0);
        match next {
            '=' => {
                self.cursor.advance();
                token!(LessEqual, start, self.cursor.len_consumed())
            }
            '<' => {
                self.cursor.advance();
                let next = self.cursor.peek(0);
                match next {
                    '=' => {
                        self.cursor.advance();
                        token!(LessLessEqual, start, self.cursor.len_consumed())
                    }
                    _ => token!(LessLess, start, self.cursor.len_consumed()),
                }
            }
            _ => token!(Less, start, self.cursor.len_consumed()),
        }
    }

    // Skips initial and ending string identifier ' || " and verifies that a string is closed
    #[inline(always)]
    fn tokenize_string(
        &mut self,
        string_category: StringCategory,
        predicate: impl FnMut(char) -> bool,
    ) -> Option<Token> {
        // we don't want the lookup to contain "
        let start = self.cursor.len_consumed();
        self.cursor.skip_while(predicate);
        if self.cursor.is_eof() {
            token!(
                Category::Unclosed(UnclosedCategory::String(string_category)),
                start,
                self.cursor.len_consumed()
            )
        } else {
            let result = {
                let raw = &self.code[Range {
                    start,
                    end: self.cursor.len_consumed(),
                }];
                match string_category {
                    StringCategory::Quotable => raw.to_owned(),
                    StringCategory::Unquotable => {
                        let mut string = raw.to_string();
                        string = string.replace(r#"\n"#, "\n");
                        string = string.replace(r#"\\"#, "\\");
                        string = string.replace(r#"\""#, "\"");
                        string = string.replace(r#"\'"#, "'");
                        string = string.replace(r#"\r"#, "\r");
                        string = string.replace(r#"\t"#, "\t");
                        string
                    }
                }
            };
            let result = token!(Category::String(result), start, self.cursor.len_consumed());
            // skip "
            self.cursor.advance();
            result
        }
    }
    #[inline(always)]
    fn may_parse_ipv4(&mut self, base: Base, start: usize) -> Option<Token> {
        use Base::*;
        // IPv4Address start as Base10
        if base == Base10 && self.cursor.peek(0) == '.' && self.cursor.peek(1).is_numeric() {
            self.cursor.advance();
            self.cursor.skip_while(base.verifier());
            // verify it may be an IPv4Address
            // if the next one is a dot we are at
            // 127.0
            // and need to parse .0
            if self.cursor.peek(0) == '.' {
                if self.cursor.peek(1).is_numeric() {
                    self.cursor.advance();
                    self.cursor.skip_while(base.verifier());
                } else {
                    return token!(
                        Category::IllegalIPv4Address,
                        start,
                        self.cursor.len_consumed()
                    );
                }

                if self.cursor.peek(0) == '.' && self.cursor.peek(1).is_numeric() {
                    self.cursor.advance();
                    self.cursor.skip_while(base.verifier());
                } else {
                    return token!(
                        Category::IllegalIPv4Address,
                        start,
                        self.cursor.len_consumed()
                    );
                }
                return token!(Category::IPv4Address, start, self.cursor.len_consumed());
            } else {
                return token!(Category::IPv4Address, start, self.cursor.len_consumed());
            }
        }
        None
    }

    // checks if a number is binary, octal, base10 or hex
    #[inline(always)]
    pub fn tokenize_number(&mut self, mut start: usize, current: char) -> Option<Token> {
        use Base::*;
        let may_base = {
            if current == '0' {
                match self.cursor.peek(0) {
                    'b' => {
                        // jump over non numeric
                        self.cursor.advance();
                        // we don't need `0b` later
                        start += 2;
                        Some(Binary)
                    }
                    'x' => {
                        // jump over non numeric
                        self.cursor.advance();
                        // we don't need `0x` later
                        start += 2;
                        Some(Hex)
                    }
                    peeked if ('0'..='7').contains(&peeked) => {
                        // we don't need leading 0 later
                        start += 1;
                        Some(Octal)
                    }
                    peeked if peeked.is_alphabetic() => None,
                    _ => Some(Base10),
                }
            } else {
                Some(Base10)
            }
        };
        if let Some(base) = may_base {
            self.cursor.skip_while(base.verifier());
            match self.may_parse_ipv4(base, start) {
                Some(token) => Some(token),
                None => {
                    // we verify that the cursor actually moved to prevent scenarios like
                    // 0b without any actual number in it
                    if start == self.cursor.len_consumed() {
                        token!(Category::IllegalNumber(base), start, start)
                    } else {
                        match i64::from_str_radix(
                            &self.code[Range {
                                start,
                                end: self.cursor.len_consumed(),
                            }],
                            base.radix(),
                        ) {
                            Ok(num) => {
                                token!(Category::Number(num), start, self.cursor.len_consumed())
                            }
                            Err(_) => token!(Category::IllegalNumber(base), start, start),
                        }
                    }
                }
            }
        } else {
            token!(Category::UnknownBase, start, self.cursor.len_consumed())
        }
    }

    // Checks if an identifier is a Keyword or not
    #[inline(always)]
    fn tokenize_identifier(&mut self, start: usize) -> Option<Token> {
        self.cursor
            .skip_while(|c| c.is_alphabetic() || c == '_' || c.is_numeric());
        let end = self.cursor.len_consumed();
        let lookup = self.lookup(Range { start, end });
        let cat = {
            if lookup != "x" {
                let keyword = IdentifierType::new(lookup);
                Category::Identifier(keyword)
            } else {
                self.cursor.skip_while(|c| c.is_whitespace());
                if self.cursor.peek(0).is_numeric() {
                    Category::X
                } else {
                    Category::Identifier(IdentifierType::Undefined(lookup.to_owned()))
                }
            }
        };
        token!(cat, start, end)
    }
}

// Is used to simplify cases for double_tokens, instead of having to rewrite each match case for each double_token
// this macro can be used:
//'+' => double_token!(self.cursor, start, '+', '+', PlusPlus, '=', PlusEqual),
// within the Iterator implementation of Tokenizer
macro_rules! two_symbol_token {
    ($cursor:expr, $start:tt, $single_symbol:tt, $($matching_char:tt, $two_symbol_token:expr ), *) => {
        {
            let next = $cursor.peek(0);
            match next {
                $($matching_char => {
                  $cursor.advance();
                  token!($two_symbol_token, $start, $cursor.len_consumed())
                }, )*
                _ => token!($single_symbol, $start, $cursor.len_consumed()),
            }
        }
    };
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Category::*;
        self.cursor.skip_while(|c| c.is_whitespace());
        let start = self.cursor.len_consumed();
        match self.cursor.advance()? {
            '(' => token!(LeftParen, start, self.cursor.len_consumed()),
            ')' => token!(RightParen, start, self.cursor.len_consumed()),
            '[' => token!(LeftBrace, start, self.cursor.len_consumed()),
            ']' => token!(RightBrace, start, self.cursor.len_consumed()),
            '{' => token!(LeftCurlyBracket, start, self.cursor.len_consumed()),
            '}' => token!(RightCurlyBracket, start, self.cursor.len_consumed()),
            ',' => token!(Comma, start, self.cursor.len_consumed()),
            '.' => token!(Dot, start, self.cursor.len_consumed()),
            '#' => {
                self.cursor.skip_while(|c| c != '\n');
                token!(Category::Comment, start, self.cursor.len_consumed())
            }
            '-' => two_symbol_token!(self.cursor, start, Minus, '-', MinusMinus, '=', MinusEqual),
            '+' => two_symbol_token!(self.cursor, start, Plus, '+', PlusPlus, '=', PlusEqual),
            '%' => two_symbol_token!(self.cursor, start, Percent, '=', PercentEqual),
            ';' => token!(Semicolon, start, self.cursor.len_consumed()),
            '/' => two_symbol_token!(self.cursor, start, Slash, '=', SlashEqual), /* self.tokenize_slash(start), */
            '*' => two_symbol_token!(self.cursor, start, Star, '*', StarStar, '=', StarEqual),
            ':' => token!(DoublePoint, start, self.cursor.len_consumed()),
            '~' => token!(Tilde, start, self.cursor.len_consumed()),
            '&' => two_symbol_token!(self.cursor, start, Ampersand, '&', AmpersandAmpersand),
            '|' => two_symbol_token!(self.cursor, start, Pipe, '|', PipePipe),
            '^' => token!(Caret, start, self.cursor.len_consumed()),
            '!' => two_symbol_token!(self.cursor, start, Bang, '=', BangEqual, '~', BangTilde),
            '=' => two_symbol_token!(self.cursor, start, Equal, '=', EqualEqual, '~', EqualTilde),
            '>' => self.tokenize_greater(),
            '<' => self.tokenize_less(),
            '"' => self.tokenize_string(StringCategory::Unquotable, |c| c != '"'),
            '\'' => {
                let mut back_slash = false;
                self.tokenize_string(StringCategory::Quotable, |c| {
                    if !back_slash && c == '\'' {
                        false
                    } else {
                        back_slash = !back_slash && c == '\\';
                        true
                    }
                })
            }

            current if ('0'..='9').contains(&current) => self.tokenize_number(start, current),
            current if current.is_alphabetic() || current == '_' => self.tokenize_identifier(start),
            _ => token!(UnknownSymbol, start, self.cursor.len_consumed()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_token(input: (Category, usize, usize)) -> Token {
        let (category, start, end) = input;
        Token {
            category,
            position: (start, end),
        }
    }

    // use macro instead of a method to have correct line numbers on failure
    macro_rules! verify_tokens {
        ($code:expr, $expected:expr) => {{
            let tokenizer = Tokenizer::new($code);
            let actual: Vec<Token> = tokenizer.clone().collect();
            let expected: Vec<Token> = $expected.iter().map(|x| build_token(x.clone())).collect();
            assert_eq!(actual, expected);
            (tokenizer, actual)
        }};
    }

    #[test]
    fn skip_white_space() {
        verify_tokens!("     (       ", vec![(Category::LeftParen, 5, 6)]);
    }

    #[test]
    fn single_symbol_tokens() {
        verify_tokens!("(", vec![(Category::LeftParen, 0, 1)]);
        verify_tokens!(")", vec![(Category::RightParen, 0, 1)]);
        verify_tokens!("[", vec![(Category::LeftBrace, 0, 1)]);
        verify_tokens!("]", vec![(Category::RightBrace, 0, 1)]);
        verify_tokens!("{", vec![(Category::LeftCurlyBracket, 0, 1)]);
        verify_tokens!("}", vec![(Category::RightCurlyBracket, 0, 1)]);
        verify_tokens!(",", vec![(Category::Comma, 0, 1)]);
        verify_tokens!(".", vec![(Category::Dot, 0, 1)]);
        verify_tokens!("-", vec![(Category::Minus, 0, 1)]);
        verify_tokens!("+", vec![(Category::Plus, 0, 1)]);
        verify_tokens!("%", vec![(Category::Percent, 0, 1)]);
        verify_tokens!(";", vec![(Category::Semicolon, 0, 1)]);
        verify_tokens!("/", vec![(Category::Slash, 0, 1)]);
        verify_tokens!("*", vec![(Category::Star, 0, 1)]);
        verify_tokens!(":", vec![(Category::DoublePoint, 0, 1)]);
        verify_tokens!("~", vec![(Category::Tilde, 0, 1)]);
        verify_tokens!("&", vec![(Category::Ampersand, 0, 1)]);
        verify_tokens!("|", vec![(Category::Pipe, 0, 1)]);
        verify_tokens!("^", vec![(Category::Caret, 0, 1)]);
    }

    #[test]
    fn two_symbol_tokens() {
        verify_tokens!("&", vec![(Category::Ampersand, 0, 1)]);
        verify_tokens!("&&", vec![(Category::AmpersandAmpersand, 0, 2)]);
        verify_tokens!("|", vec![(Category::Pipe, 0, 1)]);
        verify_tokens!("||", vec![(Category::PipePipe, 0, 2)]);
        verify_tokens!("!", vec![(Category::Bang, 0, 1)]);
        verify_tokens!("!=", vec![(Category::BangEqual, 0, 2)]);
        verify_tokens!("!~", vec![(Category::BangTilde, 0, 2)]);
        verify_tokens!("=", vec![(Category::Equal, 0, 1)]);
        verify_tokens!("==", vec![(Category::EqualEqual, 0, 2)]);
        verify_tokens!("=~", vec![(Category::EqualTilde, 0, 2)]);
        verify_tokens!(">", vec![(Category::Greater, 0, 1)]);
        verify_tokens!(">>", vec![(Category::GreaterGreater, 0, 2)]);
        verify_tokens!(">=", vec![(Category::GreaterEqual, 0, 2)]);
        verify_tokens!("><", vec![(Category::GreaterLess, 0, 2)]);
        verify_tokens!("<", vec![(Category::Less, 0, 1)]);
        verify_tokens!("<<", vec![(Category::LessLess, 0, 2)]);
        verify_tokens!("<=", vec![(Category::LessEqual, 0, 2)]);
        verify_tokens!("-", vec![(Category::Minus, 0, 1)]);
        verify_tokens!("--", vec![(Category::MinusMinus, 0, 2)]);
        verify_tokens!("+", vec![(Category::Plus, 0, 1)]);
        verify_tokens!("+=", vec![(Category::PlusEqual, 0, 2)]);
        verify_tokens!("++", vec![(Category::PlusPlus, 0, 2)]);
        verify_tokens!("/", vec![(Category::Slash, 0, 1)]);
        verify_tokens!("/=", vec![(Category::SlashEqual, 0, 2)]);
        verify_tokens!("*", vec![(Category::Star, 0, 1)]);
        verify_tokens!("**", vec![(Category::StarStar, 0, 2)]);
        verify_tokens!("*=", vec![(Category::StarEqual, 0, 2)]);
    }

    #[test]
    fn three_symbol_tokens() {
        verify_tokens!(">>>", vec![(Category::GreaterGreaterGreater, 0, 3)]);
        verify_tokens!(">>=", vec![(Category::GreaterGreaterEqual, 0, 3)]);
        verify_tokens!(">!<", vec![(Category::GreaterBangLess, 0, 3)]);
        verify_tokens!("<<=", vec![(Category::LessLessEqual, 0, 3)]);
    }

    #[test]
    fn four_symbol_tokens() {
        verify_tokens!(">>>=", vec![(Category::GreaterGreaterGreaterEqual, 0, 4)]);
    }

    #[test]
    fn unquotable_string() {
        use StringCategory::*;
        let code = "\"hello I am a closed string\\\"";
        let (tokenizer, result) = verify_tokens!(
            code,
            vec![(
                Category::String("hello I am a closed string\\".to_owned()),
                1,
                28
            )]
        );
        assert_eq!(
            tokenizer.lookup(Range::from(&result[0])),
            "hello I am a closed string\\"
        );
        let code = "\"hello I am a unclosed string\\";
        verify_tokens!(
            code,
            vec![(
                Category::Unclosed(UnclosedCategory::String(Unquotable)),
                1,
                30
            )]
        );
    }

    #[test]
    fn quotable_string() {
        use StringCategory::*;
        let code = "'Hello \\'you\\'!'";
        verify_tokens!(
            code,
            vec![(Category::String("Hello \\'you\\'!".to_owned()), 1, 15)]
        );
        let code = "'Hello \\'you\\'!\\'";
        verify_tokens!(
            code,
            vec![(
                Category::Unclosed(UnclosedCategory::String(Quotable)),
                1,
                17
            )]
        );
    }

    #[test]
    fn numbers() {
        use Base::*;
        use Category::*;
        verify_tokens!("0", vec![(Number(0), 0, 1)]);
        verify_tokens!("0b01", vec![(Number(1), 2, 4)]);
        verify_tokens!("1234567890", vec![(Number(1234567890), 0, 10)]);
        // TODO remove float
        //verify_tokens!("0.123456789", vec![(Number(Base10), 0, 11)]);
        verify_tokens!("012345670", vec![(Number(2739128), 1, 9)]);
        verify_tokens!(
            "0x1234567890ABCDEF",
            vec![(Number(1311768467294899695), 2, 18)]
        );
        // That would be later illegal because a number if followed by a number
        // but within tokenizing I think it is the best to ignore that and let it be handled by AST
        verify_tokens!("0b02", vec![(Number(0), 2, 3), (Number(2), 3, 4)]);
        verify_tokens!(
            "0b2",
            vec![(IllegalNumber(Binary), 2, 2), (Number(2), 2, 3)]
        );
    }

    #[test]
    fn single_line_comments() {
        use Category::*;
        verify_tokens!(
            "# this is a comment\n;",
            vec![(Comment, 0, 19), (Semicolon, 20, 21)]
        );
    }

    #[test]
    fn identifier() {
        use Category::*;
        use IdentifierType::*;
        verify_tokens!(
            "hel_lo",
            vec![(Identifier(Undefined("hel_lo".to_owned())), 0, 6)]
        );
        verify_tokens!(
            "_hello",
            vec![(Identifier(Undefined("_hello".to_owned())), 0, 6)]
        );
        verify_tokens!(
            "_h4llo",
            vec![(Identifier(Undefined("_h4llo".to_owned())), 0, 6)]
        );
        verify_tokens!(
            "4_h4llo",
            vec![
                (Number(4), 0, 1),
                (Identifier(Undefined("_h4llo".to_owned())), 1, 7)
            ]
        );
    }

    #[test]
    fn keywords() {
        use Category::*;
        use IdentifierType::*;
        verify_tokens!("for", vec![(Identifier(For), 0, 3)]);
        verify_tokens!("foreach", vec![(Identifier(ForEach), 0, 7)]);
        verify_tokens!("if", vec![(Identifier(If), 0, 2)]);
        verify_tokens!("else", vec![(Identifier(Else), 0, 4)]);
        verify_tokens!("while", vec![(Identifier(While), 0, 5)]);
        verify_tokens!("repeat", vec![(Identifier(Repeat), 0, 6)]);
        verify_tokens!("until", vec![(Identifier(Until), 0, 5)]);
        verify_tokens!("local_var", vec![(Identifier(LocalVar), 0, 9)]);
        verify_tokens!("global_var", vec![(Identifier(GlobalVar), 0, 10)]);
        verify_tokens!("NULL", vec![(Identifier(Null), 0, 4)]);
        verify_tokens!("return", vec![(Identifier(Return), 0, 6)]);
        verify_tokens!("include", vec![(Identifier(Include), 0, 7)]);
        verify_tokens!("exit", vec![(Identifier(Exit), 0, 4)]);
    }

    #[test]
    fn string_quoting() {
        use Category::*;
        verify_tokens!(
            r###"'webapps\\appliance\\'"###,
            vec![(String("webapps\\\\appliance\\\\".to_owned()), 1, 21)]
        );
    }

    #[test]
    fn simplified_ipv4_address() {
        use Category::*;
        verify_tokens!("10.187.76.12", vec![(IPv4Address, 0, 12)]);
    }

    #[test]
    fn repeat_x_times() {
        use Category::*;
        verify_tokens!(
            "x() x 10;",
            vec![
                (Identifier(IdentifierType::Undefined("x".to_owned())), 0, 1),
                (LeftParen, 1, 2),
                (RightParen, 2, 3),
                (X, 4, 5),
                (Number(10), 6, 8),
                (Semicolon, 8, 9),
            ]
        );
    }

    #[test]
    fn tokenize_description_block() {
        use Category::*;
        use IdentifierType::*;

        let code = r#"
if(description)
{
  script_oid("1.3.6.1.4.1.25623.1.0.99999");
  exit(0);
}

j = 123;
j >>>= 8;
display(j);
exit(1);
"#;
        verify_tokens!(
            code,
            vec![
                (Identifier(If), 1, 3),
                (LeftParen, 3, 4), // start expression block
                (Identifier(Undefined("description".to_owned())), 4, 15), // verify is description is true
                (RightParen, 15, 16),                                     // end expression block
                (LeftCurlyBracket, 17, 18),                               // start execution block
                (Identifier(Undefined("script_oid".to_owned())), 21, 31), // lookup function script_oid
                (LeftParen, 31, 32), // start parameter expression block
                (String("1.3.6.1.4.1.25623.1.0.99999".to_owned()), 33, 60), // resolve prime to "1.3.6.1.4.1.25623.1.0.99999"
                (RightParen, 61, 62),                                       // end expression block
                (Semicolon, 62, 63),                                        // finish execution
                (Identifier(Exit), 66, 70),                                 // lookup keyword exit
                (LeftParen, 70, 71),         // start parameter expression block
                (Number(0), 71, 72),         // call exit with 0
                (RightParen, 72, 73),        // end expression block
                (Semicolon, 73, 74),         // finish execution
                (RightCurlyBracket, 75, 76), // finish expression block
                (Identifier(Undefined("j".to_owned())), 78, 79), // lookup j
                (Equal, 80, 81),             // assign to j
                (Number(123), 82, 85),       // number 123
                (Semicolon, 85, 86),         // finish execution
                (Identifier(Undefined("j".to_owned())), 87, 88), // lookup j
                (GreaterGreaterGreaterEqual, 89, 93), // shift j and assign to j
                (Number(8), 94, 95),         // 8
                (Semicolon, 95, 96),         // finish execution
                (Identifier(Undefined("display".to_owned())), 97, 104), // lookup display
                (LeftParen, 104, 105),       // start parameter expression block
                (Identifier(Undefined("j".to_owned())), 105, 106), // resolve j primitive
                (RightParen, 106, 107),      // finish parameter expression block
                (Semicolon, 107, 108),       // finish execution
                (Identifier(Exit), 109, 113), // lookup keyword exit
                (LeftParen, 113, 114),       // start parameter expression block
                (Number(1), 114, 115),       // call exit with 1
                (RightParen, 115, 116),      // finish parameter expression block
                (Semicolon, 116, 117)        // finish execution
            ]
        );
    }
}
