use super::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum PieceKind {
    Empty,
    Pawn,
    Cannon,
    King,
    Advisor,
    Bishop,
    Knight,
    Rook,
}

impl TryFrom<char> for PieceKind {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Error> {
        match value {
            '-' => Ok(PieceKind::Empty),
            'p' => Ok(PieceKind::Pawn),
            'c' => Ok(PieceKind::Cannon),
            'k' => Ok(PieceKind::King),
            'a' => Ok(PieceKind::Advisor),
            'b' => Ok(PieceKind::Bishop),
            'n' => Ok(PieceKind::Knight),
            'r' => Ok(PieceKind::Rook),
            _ => Err(Error),
        }
    }
}

impl From<PieceKind> for char {
    fn from(val: PieceKind) -> Self {
        match val {
            PieceKind::Empty => '-',
            PieceKind::Pawn => 'p',
            PieceKind::Cannon => 'c',
            PieceKind::King => 'k',
            PieceKind::Advisor => 'a',
            PieceKind::Bishop => 'b',
            PieceKind::Knight => 'n',
            PieceKind::Rook => 'r',
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum PieceColor {
    Red,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            PieceColor::Red => PieceColor::Black,
            PieceColor::Black => PieceColor::Red,
        }
    }
}

impl TryFrom<char> for PieceColor {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Error> {
        match value {
            'r' => Ok(PieceColor::Red),
            'b' => Ok(PieceColor::Black),
            _ => Err(Error),
        }
    }
}

impl From<PieceColor> for char {
    fn from(val: PieceColor) -> Self {
        match val {
            PieceColor::Red => 'r',
            PieceColor::Black => 'b',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self { kind, color }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(self) -> bool {
        self.kind == PieceKind::Empty
    }

    pub fn is_enemy(self, other: Piece) -> bool {
        !self.is_empty() && !other.is_empty() && self.color != other.color
    }

    pub fn is_ally(self, other: Piece) -> bool {
        !self.is_empty() && !other.is_empty() && self.color == other.color
    }

    pub fn is_steppable(self, other: Piece) -> bool {
        other.is_empty() || self.color != other.color
    }

    pub fn is_kind(self, kind: PieceKind) -> bool {
        self.kind == kind
    }

    pub fn is_color(self, color: PieceColor) -> bool {
        !self.is_empty() && self.color == color
    }

    pub fn color(self) -> Option<PieceColor> {
        if self.is_empty() {
            None
        } else {
            Some(self.color)
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            kind: PieceKind::Empty,
            color: PieceColor::Red,
        }
    }
}

impl TryFrom<&str> for Piece {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        let mut chars = value.chars();
        let kind = chars.next().ok_or(Error)?.try_into()?;
        let color = chars.next().ok_or(Error)?.try_into()?;
        Ok(Piece { kind, color })
    }
}

impl From<Piece> for String {
    fn from(val: Piece) -> Self {
        format!("{}{}", char::from(val.kind), char::from(val.color))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(isize, isize);

impl Position {
    pub const fn new(rank: usize, file: usize) -> Self {
        Self(rank as isize, file as isize)
    }

    pub const fn rank(self) -> usize {
        self.0 as usize
    }

    pub const fn file(self) -> usize {
        self.1 as usize
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Position {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Neg for Position {
    type Output = Position;
    fn neg(self) -> Position {
        Position(-self.0, -self.1)
    }
}

impl From<Position> for String {
    fn from(value: Position) -> Self {
        format!(
            "{}{}",
            (b'0' + value.rank() as u8) as char,
            (b'0' + value.file() as u8) as char
        )
    }
}

impl From<String> for Position {
    fn from(value: String) -> Self {
        let mut chars = value.chars();
        let rank = chars.next().unwrap() as u8 - b'0';
        let file = chars.next().unwrap() as u8 - b'0';
        Position(rank as isize, file as isize)
    }
}
