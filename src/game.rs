use super::board::Board;
use super::piece::Piece;
use super::piece;

use web_sys::console;

pub struct Game {
    board: Board,
    piece: Option<Piece>,
    _next_piece: Option<Piece>,
    _shadow_piece: Option<Piece>,
    score: u32,
}

impl Game {
    pub fn new() -> Self {
        Game{
            board: Board::new(20, 10),
            piece: None,
            _next_piece: None,
            _shadow_piece: None,
            score: 0,
        }
    }

    pub fn pause(&mut self) {
        console::log_1(&"Pause".into());
    }

    pub fn move_down(&mut self) {
        console::log_1(&"move_down".into());
        let piece = match self.piece.as_mut() {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.move_down();
        if self.board.is_legal(&piece) {
            console::log_1(&"legal".into());
            return
        }
        piece.revert_move_down();
        self.board.freeze(self.piece.take().unwrap());
        let cleared_lines = self.board.clear_lines();
        self.update_score(cleared_lines);
        self.spawn();
    }

    pub fn move_left(&mut self) {
        console::log_1(&"move_left".into());
        let piece = match self.piece.as_mut() {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.move_left();
        if self.board.is_legal(&piece) {
            console::log_1(&"legal".into());
            return
        }
        piece.move_right();
    }

    pub fn move_right(&mut self) {
        console::log_1(&"move_right".into());
        let piece = match self.piece.as_mut() {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.move_right();
        if self.board.is_legal(&piece) {
            console::log_1(&"legal".into());
            return
        }
        piece.move_left();
    }

    pub fn rotate_clockwise(&mut self) {
        console::log_1(&"rotate_clockwise".into());
        let piece = match self.piece.as_mut() {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.rotate_clockwise();
        if self.board.is_legal(&piece) {
            console::log_1(&"legal".into());
            return
        }
        piece.rotate_anticlockwise();
    }

    pub fn run(&mut self) {
        console::log_1(&"Run".into());
        self.spawn();
    }

    fn create_random_piece() -> Piece {
        let piece = Piece::new(17, 3, rand::random());
        piece
    }

    fn spawn(&mut self) {
        let piece = Self::create_random_piece();
        if !self.board.is_legal(&piece) {
            self.game_over();
            return
        }
        self.piece = Some(piece);
        console::log_1(&"spawned".into());
    }

    fn game_over(&mut self) {
        console::log_1(&"game over".into());
    }

    fn update_score(&mut self, cleared_lines: u32) {
        self.score += cleared_lines;
        console::log_1(&format!("score: {}", self.score).into());
    }
}

impl Game {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }
}
