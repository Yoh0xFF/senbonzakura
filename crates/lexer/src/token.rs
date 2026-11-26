/// A token: its kind, the source range it covers, and flags.
///
/// Tokens hold byte offsets, not text. [`SourceFile::text_of`](super::SourceFile::text_of) and
/// [`SourceFile::locate`](super::SourceFile::locate) turn them into text and positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
  pub kind: Kind,
  /// Offset of the first byte.
  pub start: usize,
  /// Offset just past the last byte; equal to `start` for [`Kind::Eof`].
  pub end: usize,
  /// The flag constants below, combined with `|`.
  pub flags: u8,
}

impl Token {
  /// A line break appears between the previous token, or the start of the input, and this one,
  /// possibly inside a block comment.
  pub const NEWLINE_BEFORE: u8 = 1;
  /// Whitespace or a comment appears between the previous token, or the start, and this one.
  pub const SPACE_BEFORE: u8 = 1 << 1;
  /// The literal has no closing quote; a diagnostic has already been reported.
  pub const UNTERMINATED: u8 = 1 << 2;
  /// The literal contains escape sequences, so its value differs from its source text.
  pub const HAS_ESCAPES: u8 = 1 << 3;

  /// # Panics
  ///
  /// If `end` is before `start`.
  pub fn new(kind: Kind, start: usize, end: usize, flags: u8) -> Token {
    assert!(start <= end, "invalid token range {start}..{end}");
    Token { kind, start, end, flags }
  }

  pub fn eof(offset: usize, flags: u8) -> Token {
    Token::new(Kind::Eof, offset, offset, flags)
  }

  pub fn len(&self) -> usize {
    self.end - self.start
  }

  pub fn is_empty(&self) -> bool {
    self.start == self.end
  }

  pub fn has(&self, flag: u8) -> bool {
    self.flags & flag != 0
  }
}

/// A problem found while lexing, parsing or checking, covering offsets `start` up to (not
/// including) `end`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
  pub start: usize,
  pub end: usize,
  pub message: String,
}

impl Diagnostic {
  pub fn new(start: usize, end: usize, message: impl Into<String>) -> Diagnostic {
    Diagnostic { start, end, message: message.into() }
  }
}

/// How a kind with a fixed spelling is scanned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Category {
  /// A reserved word, scanned like an identifier.
  Keyword,
  /// An operator or punctuation mark, scanned by longest match.
  Operator,
}

macro_rules! kinds {
    ($($kind:ident $(= $spelling:literal $category:ident)?,)*) => {
        /// What a token is. Keywords and operators each get their own kind, so the parser never
        /// compares strings.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Kind {
            $($kind,)*
        }

        impl Kind {
            pub const ALL: &'static [Kind] = &[$(Kind::$kind,)*];

            /// The fixed text of a keyword or operator, or `None` for kinds whose text varies.
            pub fn spelling(self) -> Option<&'static str> {
                match self {
                    $($(Kind::$kind => Some($spelling),)?)*
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }

            pub(crate) fn category(self) -> Option<Category> {
                match self {
                    $($(Kind::$kind => Some(Category::$category),)?)*
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        }
    };
}

kinds! {
    // Names and literals
    Identifier,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // Keywords
    Fn = "fn" Keyword,
    Var = "var" Keyword,
    If = "if" Keyword,
    Else = "else" Keyword,
    Loop = "loop" Keyword,
    In = "in" Keyword,
    Return = "return" Keyword,
    Break = "break" Keyword,
    Continue = "continue" Keyword,
    True = "true" Keyword,
    False = "false" Keyword,
    As = "as" Keyword,

    // Primitive types
    Byte = "byte" Keyword,
    Short = "short" Keyword,
    Int = "int" Keyword,
    Size = "size" Keyword,
    Long = "long" Keyword,
    Ubyte = "ubyte" Keyword,
    Ushort = "ushort" Keyword,
    Uint = "uint" Keyword,
    Usize = "usize" Keyword,
    Ulong = "ulong" Keyword,
    Float = "float" Keyword,
    Double = "double" Keyword,
    Char = "char" Keyword,
    Bool = "bool" Keyword,
    String = "string" Keyword,

    // Punctuation
    LeftParen = "(" Operator,
    RightParen = ")" Operator,
    LeftBrace = "{" Operator,
    RightBrace = "}" Operator,
    LeftBracket = "[" Operator,
    RightBracket = "]" Operator,
    Comma = "," Operator,
    Semicolon = ";" Operator,
    Colon = ":" Operator,
    Dot = "." Operator,
    DotDot = ".." Operator,
    Arrow = "->" Operator,

    // Arithmetic
    Plus = "+" Operator,
    Minus = "-" Operator,
    Star = "*" Operator,
    Slash = "/" Operator,
    Percent = "%" Operator,

    // Assignment
    Eq = "=" Operator,
    PlusEq = "+=" Operator,
    MinusEq = "-=" Operator,
    StarEq = "*=" Operator,
    SlashEq = "/=" Operator,
    PercentEq = "%=" Operator,

    // Increment and decrement, postfix only
    PlusPlus = "++" Operator,
    MinusMinus = "--" Operator,

    // Comparison
    EqEq = "==" Operator,
    BangEq = "!=" Operator,
    Less = "<" Operator,
    LessEq = "<=" Operator,
    Greater = ">" Operator,
    GreaterEq = ">=" Operator,

    // Logical
    AndAnd = "&&" Operator,
    OrOr = "||" Operator,
    Bang = "!" Operator,

    // A character that starts no token; a diagnostic has already been reported.
    Error,

    // The zero-width token at the end of the input.
    Eof,
}

impl Kind {
  pub(crate) fn is_keyword(self) -> bool {
    self.category() == Some(Category::Keyword)
  }

  pub(crate) fn is_operator(self) -> bool {
    self.category() == Some(Category::Operator)
  }
}
