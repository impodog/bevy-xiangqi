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

    pub fn turn(&self) -> PieceColor {
        self.turn
    }

    pub fn next_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    fn test_move(&self, from: Position, to: Position) -> Option<Position> {
        if to
            .legal()
            .map(|to| self.get(from).is_steppable(self.get(to)))
            .unwrap_or_default()
        {
            Some(to)
        } else {
            None
        }
    }

    fn reachable_pawn(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::Pawn));
        let dir = match piece.color().unwrap() {
            PieceColor::Red => MoveDir::Up,
            PieceColor::Black => MoveDir::Down,
        };
        // If the pawn crosses the river
        if (from.rank_int() > 4) ^ (dir == MoveDir::Up) {
            if let Some(to) = self.test_move(from, from + MoveDir::Left.into()) {
                result.insert(to);
            }
            if let Some(to) = self.test_move(from, from + MoveDir::Right.into()) {
                result.insert(to);
            }
        }
        // Also move forward
        if let Some(to) = self.test_move(from, from + dir.into()) {
            result.insert(to);
        }
    }

    fn reachable_cannon(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::Cannon));
        for dir in MOVE_DIRS {
            let mut to = from;
            loop {
                to = to + dir.into();
                if let Some(to) = to.legal() {
                    if self.get(to).is_empty() {
                        result.insert(to);
                    } else {
                        // here, the non-empty square is skipped in the following loop
                        break;
                    }
                } else {
                    break;
                }
            }
            loop {
                to = to + dir.into();
                if let Some(to) = to.legal() {
                    let to_piece = self.get(to);
                    if !to_piece.is_empty() {
                        if to_piece.is_enemy(piece) {
                            result.insert(to);
                        }
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn reachable_king(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::King));
        let (dir, max_rank) = match piece.color().unwrap() {
            PieceColor::Red => (MoveDir::Up, 2),
            // Black is 6 because the xor operator with change "<=" into ">"
            PieceColor::Black => (MoveDir::Down, 6),
        };
        for dir in MOVE_DIRS {
            if let Some(to) = self.test_move(from, from + dir.into()) {
                if 4 <= to.file()
                    && to.file() <= 6
                    && ((to.rank() <= max_rank) ^ piece.is_color(PieceColor::Red))
                {
                    result.insert(to);
                }
            }
        }
        let mut to = from;
        loop {
            to = to + dir.into();
            if let Some(to) = to.legal() {
                let to_piece = self.get(to);
                if !to_piece.is_empty() {
                    if to_piece.is_kind(PieceKind::King) {
                        result.insert(to);
                    }
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn reachable_advisor(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::Advisor));
        let max_rank = match piece.color().unwrap() {
            PieceColor::Red => 2,
            // See above
            PieceColor::Black => 6,
        };
        for dir in DIAG_DIRS {
            if let Some(to) = self.test_move(from, from + dir.into()) {
                if 3 <= to.rank()
                    && to.rank() <= 5
                    && ((to.file() <= max_rank) ^ (piece.is_color(PieceColor::Red)))
                {
                    result.insert(to);
                }
            }
        }
    }

    fn reachable_bishop(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::Bishop));
        for dir in DIAG_DIRS {
            if let Some(mid) = (from + dir.into()).legal() {
                if self.get(mid).is_empty() {
                    if let Some(to) = self.test_move(from, from + Into::<Position>::into(dir) * 2) {
                        result.insert(to);
                    }
                }
            }
        }
    }

    fn reachable_knight(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::Knight));
        for dir in MOVE_DIRS {
            for co_dir in dir.corresponding() {
                if let Some(mid) = (from + dir.into()).legal() {
                    if self.get(mid).is_empty() {
                        if let Some(to) = self.test_move(from, from + dir.into() + co_dir.into()) {
                            result.insert(to);
                        }
                    }
                }
            }
        }
    }

    fn reachable_rook(&self, from: Position, result: &mut HashSet<Position>) {
        let piece = self.get(from);
        debug_assert!(piece.is_kind(PieceKind::Rook));
        for dir in MOVE_DIRS {
            let mut to = from;
            loop {
                to = to + dir.into();
                if let Some(to) = to.legal() {
                    let to_piece = self.get(to);
                    if !to_piece.is_empty() {
                        if to_piece.is_enemy(piece) {
                            result.insert(to);
                        }
                        break;
                    } else {
                        result.insert(to);
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn reachable(&self, from: Position) -> HashSet<Position> {
        let mut result = HashSet::new();
        match self.get(from).kind() {
            PieceKind::Empty => {}
            PieceKind::Pawn => self.reachable_pawn(from, &mut result),
            PieceKind::Cannon => self.reachable_cannon(from, &mut result),
            PieceKind::King => self.reachable_king(from, &mut result),
            PieceKind::Advisor => self.reachable_advisor(from, &mut result),
            PieceKind::Bishop => self.reachable_bishop(from, &mut result),
            PieceKind::Knight => self.reachable_knight(from, &mut result),
            PieceKind::Rook => self.reachable_rook(from, &mut result),
        }
        result
    }

    pub fn king(&self, color: PieceColor) -> Position {
        self.kings[Into::<u8>::into(color) as usize]
    }

    pub fn is_check(&self) -> bool {
        let king = self.king(self.turn());
        for (r, file) in self.content.iter().enumerate() {
            for (f, piece) in file.iter().enumerate() {
                let pos = Position::new(r, f);
                if piece.is_color(self.turn().opposite()) {
                    let reachable = self.reachable(pos);
                    if reachable.contains(&king) {
                        return true;
                    }
                }
            }
        }
        false
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

impl TryFrom<&str> for Board {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        let mut content = Vec::new();
        (0..RANKS).for_each(|_| content.push(Vec::new()));
        let mut iter = value.chars();
        for file in content.iter_mut() {
            for _file in 0..FILES {
                let str = format!("{}{}", iter.next().ok_or(Error)?, iter.next().ok_or(Error)?);
                let piece = str.as_str().try_into()?;
                file.push(piece);
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
