use super::*;

#[derive(Debug, Clone)]
pub struct Board {
    content: Vec<Vec<Piece>>,
    kings: Vec<Position>,
    turn: PieceColor,
}

impl Default for Board {
    fn default() -> Self {
        let mut content = Vec::new();
        (0..RANKS).for_each(|_| content.push(Vec::new()));
        for (color, rank) in [(PieceColor::Red, 0), (PieceColor::Black, 9)] {
            for kind in [
                PieceKind::Rook,
                PieceKind::Knight,
                PieceKind::Bishop,
                PieceKind::Advisor,
                PieceKind::King,
                PieceKind::Advisor,
                PieceKind::Bishop,
                PieceKind::Knight,
                PieceKind::Rook,
            ] {
                content[rank].push(Piece::new(kind, color));
            }
        }
        for (color, rank) in [(PieceColor::Red, 2), (PieceColor::Black, 7)] {
            for file in 0..FILES {
                content[rank].push(Piece::new(
                    if file == 1 || file == 7 {
                        PieceKind::Cannon
                    } else {
                        PieceKind::Empty
                    },
                    color,
                ));
            }
        }
        for (color, rank) in [(PieceColor::Red, 3), (PieceColor::Black, 6)] {
            for file in 0..FILES {
                content[rank].push(Piece::new(
                    if file % 2 == 0 {
                        PieceKind::Pawn
                    } else {
                        PieceKind::Empty
                    },
                    color,
                ));
            }
        }
        for rank in [1, 4, 5, 8] {
            for _file in 0..FILES {
                content[rank].push(Piece::empty());
            }
        }

        let kings = vec![Position::new(0, 4), Position::new(9, 4)];

        Self {
            content,
            kings,
            turn: PieceColor::Red,
        }
    }
}

impl Board {
    pub fn get(&self, pos: Position) -> Piece {
        self.content[pos.rank()][pos.file()]
    }

    pub fn get_mut(&mut self, pos: Position) -> &mut Piece {
        &mut self.content[pos.rank()][pos.file()]
    }

    pub fn force(&mut self, from: Position, to: Position) {
        let piece = std::mem::take(self.get_mut(from));
        let _ = std::mem::replace(self.get_mut(to), piece);
        if piece.is_kind(PieceKind::King) {
            self.kings[Into::<u8>::into(piece.color().unwrap()) as usize] = to;
        }
    }

    pub fn next_turn(&mut self) {
        self.turn = self.turn.opposite();
    }
}

impl From<&Board> for String {
    fn from(value: &Board) -> Self {
        let mut result = String::new();
        for rank in 0..RANKS {
            for file in 0..FILES {
                let pos = Position::new(rank, file);
                let piece = value.get(pos);
                result.push_str(&Into::<String>::into(piece));
            }
        }
        result.push_str(&value.kings.len().to_string());
        result.push('/');
        result.push_str(
            &value
                .kings
                .iter()
                .map(|pos| (*pos).into())
                .collect::<Vec<String>>()
                .join(""),
        );
        result.push(value.turn.into());
        result
    }
}

impl TryFrom<String> for Board {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        let mut content = Vec::new();
        (0..RANKS).for_each(|_| content.push(Vec::new()));
        let mut iter = value.chars();
        for rank in content.iter_mut() {
            for _file in 0..FILES {
                let str = format!("{}{}", iter.next().ok_or(Error)?, iter.next().ok_or(Error)?);
                let piece = str.as_str().try_into()?;
                rank.push(piece);
            }
        }
        let mut len = String::new();
        loop {
            let c = iter.next().ok_or(Error)?;
            if c == '/' {
                break;
            } else {
                len.push(c);
            }
        }
        let len = len.parse().map_err(|_| Error)?;
        let mut kings = Vec::new();
        for _ in 0..len {
            let str = format!("{}{}", iter.next().ok_or(Error)?, iter.next().ok_or(Error)?);
            let position = str.into();
            kings.push(position);
        }
        let turn = iter.next().ok_or(Error)?.try_into()?;
        Ok(Board {
            content,
            kings,
            turn,
        })
    }
}
