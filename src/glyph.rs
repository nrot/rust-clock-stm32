use core::iter::Cycle;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbols{
    Exclamation = 0b100000000000110,
    DoubleQuotes = 0b000001000000010,
    Hash = 0b001001011001110,
    Dollar = 0b001001011101101,
    Percent = 0b011111111100100,
    Ampersand = 0b010001101011001,
    Quote = 0b000001000000000,
    LeftParen = 0b010010000000000,
    RightParen = 0b000100100000000,
    Asterisk = 0b011111111000000,
    Plus = 0b001001011000000,
    Comma = 0b000100000000000,
    Dash = 0b000000011000000,
    Dot = 0b100000000000000,
    Slash = 0b000110000000000,
    Semicolon = 0b000101000000000,
    Less = 0b010010001000000,
    Equals = 0b000000011001000,
    Greater = 0b000100110000000,
    Question = 0b101000010000011,
    At = 0b000001010111011,
    LeftBracket = 0b000000000111001,
    Backslash = 0b010000100000000,
    RightBracket = 0b000000000001111,
    Caret = 0b010100000000000,
    Underscore = 0b000000000001000,
    Apostrophe = 0b000000100000000,
    LeftBrace = 0b000100101001001,
    Pipe = 0b001001000000000,
    RightBrace = 0b010010010001001,
    Tilde = 0b000110011000000,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capitals {
    A = 0b000000011110111,
    B = 0b001001010001111,
    C = 0b000000000111001,
    D = 0b001001000001111,
    E = 0b000000001111001,
    F = 0b000000001110001,
    G = 0b000000010111101,
    H = 0b000000011110110,
    I = 0b001001000001001,
    J = 0b000000000011110,
    K = 0b010010001110000,
    L = 0b000000000111000,
    M = 0b000010100110110,
    N = 0b010000100110110,
    O = 0b000000000111111,
    P = 0b000000011110011,
    Q = 0b010000000111111,
    R = 0b010000011110011,
    S = 0b000000011101101,
    T = 0b001001000000001,
    U = 0b000000000111110,
    V = 0b000110000110000,
    W = 0b010100000110110,
    X = 0b010110100000000,
    Y = 0b000000011101110,
    Z = 0b000110000001001,
}

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lowercase {
    a = 0b001000001011000,
    b = 0b010000001111000,
    c = 0b000000011011000,
    d = 0b000100010001110,
    e = 0b000100001011000,
    f = 0b001010011000000,
    g = 0b000010010001110,
    h = 0b001000001110000,
    i = 0b001000000000000,
    j = 0b000101000010000,
    k = 0b011011000000000,
    l = 0b000000000110000,
    m = 0b001000011010100,
    n = 0b001000001010000,
    o = 0b000000011011100,
    p = 0b000000101110000,
    q = 0b000010010000110,
    r = 0b000000001010000,
    s = 0b010000010001000,
    t = 0b000000001111000,
    u = 0b000000000011100,
    v = 0b000100000010000,
    w = 0b010100000010100,
    x = 0b010110100000000,
    y = 0b000001010001110,
    z = 0b000100001001000,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Digits{
    D0 = 0b000110000111111,
    D1 = 0b000010000000110,
    D2 = 0b000000011011011,
    D3 = 0b000000010001111,
    D4 = 0b000000011100110,
    D5 = 0b010000001101001,
    D6 = 0b000000011111101,
    D7 = 0b000000000000111,
    D8 = 0b000000011111111,
    D9 = 0b000000011101111,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Glyph {
    Symbols(Symbols),
    Capitals(Capitals),
    Lowercase(Lowercase),
    Digits(Digits)
}

impl From<char> for Glyph {
    #[inline(always)]
    fn from(c: char) -> Self {
        match c {
            'A'..='Z' => Glyph::Capitals(Capitals::from(c)),
            'a'..='z' => Glyph::Lowercase(Lowercase::from(c)),
            '0'..='9' => Glyph::Digits(Digits::from(c)),
            v => {
                match Symbols::try_from(v){
                    Ok(g) => Glyph::Symbols(g),
                    Err(_) => panic!("Can`t convert cahr to glyph"),

                }
            },
        }
    }
}

impl From<char> for Digits{
    #[inline(always)]
    fn from(c: char) -> Self {
        match c {
            '0' => Digits::D0,
            '1' => Digits::D1,
            '2' => Digits::D2,
            '3' => Digits::D3,
            '4' => Digits::D4,
            '5' => Digits::D5,
            '6' => Digits::D6,
            '7' => Digits::D7,
            '8' => Digits::D8,
            '9' => Digits::D9,
            _ => panic!("Invalid digit glyph"),
        }
    }
}

impl From<char> for Capitals{
    #[inline(always)]
    fn from(c: char) -> Self {
        match c {
            'A' => Capitals::A,
            'B' => Capitals::B,
            'C' => Capitals::C,
            'D' => Capitals::D,
            'E' => Capitals::E,
            'F' => Capitals::F,
            'G' => Capitals::G,
            'H' => Capitals::H,
            'I' => Capitals::I,
            'J' => Capitals::J,
            'K' => Capitals::K,
            'L' => Capitals::L,
            'M' => Capitals::M,
            'N' => Capitals::N,
            'O' => Capitals::O,
            'P' => Capitals::P,
            'Q' => Capitals::Q,
            'R' => Capitals::R,
            'S' => Capitals::S,
            'T' => Capitals::T,
            'U' => Capitals::U,
            'V' => Capitals::V,
            'W' => Capitals::W,
            'X' => Capitals::X,
            'Y' => Capitals::Y,
            'Z' => Capitals::Z,
            _ => panic!("Invalid Capital glyph"),
        }
    }
}

impl From<char> for Lowercase{
    #[inline(always)]
    fn from(c: char) -> Self {
        match c {
            'a' => Lowercase::a,
            'b' => Lowercase::b,
            'c' => Lowercase::c,
            'd' => Lowercase::d,
            'e' => Lowercase::e,
            'f' => Lowercase::f,
            'g' => Lowercase::g,
            'h' => Lowercase::h,
            'i' => Lowercase::i,
            'j' => Lowercase::j,
            'k' => Lowercase::k,
            'l' => Lowercase::l,
            'm' => Lowercase::m,
            'n' => Lowercase::n,
            'o' => Lowercase::o,
            'p' => Lowercase::p,
            'q' => Lowercase::q,
            'r' => Lowercase::r,
            's' => Lowercase::s,
            't' => Lowercase::t,
            'u' => Lowercase::u,
            'v' => Lowercase::v,
            'w' => Lowercase::w,
            'x' => Lowercase::x,
            'y' => Lowercase::y,
            'z' => Lowercase::z,
            _ => panic!("Invalid Lowercase glyph"),
        }
    }
}

impl TryFrom<char> for Symbols{
    type Error = &'static str;
    #[inline(always)]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '!' => Ok(Symbols::Exclamation),
            '"' => Ok(Symbols::DoubleQuotes),
            '#' => Ok(Symbols::Hash),
            '$' => Ok(Symbols::Dollar),
            '%' => Ok(Symbols::Percent),
            '&' => Ok(Symbols::Ampersand),
            '\'' => Ok(Symbols::Quote),
            '(' => Ok(Symbols::LeftParen),
            ')' => Ok(Symbols::RightParen),
            '*' => Ok(Symbols::Asterisk),
            '+' => Ok(Symbols::Plus),
            ',' => Ok(Symbols::Comma),
            '-' => Ok(Symbols::Dash),
            '.' => Ok(Symbols::Dot),
            '/' => Ok(Symbols::Slash),
            ';' => Ok(Symbols::Semicolon),
            '<' => Ok(Symbols::Less),
            '=' => Ok(Symbols::Equals),
            '>' => Ok(Symbols::Greater),
            '?' => Ok(Symbols::Question),
            '@' => Ok(Symbols::At),
            '[' => Ok(Symbols::LeftBracket),
            '\\' => Ok(Symbols::Backslash),
            ']' => Ok(Symbols::RightBracket),
            '^' => Ok(Symbols::Caret),
            '_' => Ok(Symbols::Underscore),
            '`' => Ok(Symbols::Apostrophe),
            '{' => Ok(Symbols::LeftBrace),
            '|' => Ok(Symbols::Pipe),
            '}' => Ok(Symbols::RightBrace),
            '~' => Ok(Symbols::Tilde),
            _ => Err("Invalid symbol glyph"),
        }
    }
}


pub trait IterTest{
    type Item;
    fn iter_test() ->Cycle<core::slice::Iter<'static, Self::Item>>;
}

impl IterTest for Symbols{
    type Item = Symbols;

    fn iter_test() ->Cycle<core::slice::Iter<'static, Self::Item>> {
        static SYSMBOLS: &[Symbols] = &[
            Symbols::Ampersand,
            Symbols::At,
            Symbols::Backslash,
            Symbols::Caret,
            Symbols::Caret,
            Symbols::Comma,
            Symbols::Dash,
            Symbols::Dot,
            Symbols::Equals,
            Symbols::Exclamation,
            Symbols::Greater,
            Symbols::Hash,
            Symbols::LeftBrace,
            Symbols::LeftBracket,
            Symbols::LeftParen,
            Symbols::Less,
            Symbols::Pipe,
            Symbols::Plus,
            Symbols::Question,
            Symbols::RightBrace,
            Symbols::RightBracket,
            Symbols::RightParen,
            Symbols::Semicolon,
            Symbols::Slash,
            Symbols::Tilde,
            Symbols::Underscore,
        ];
        SYSMBOLS.iter().cycle()
    }
    
}
