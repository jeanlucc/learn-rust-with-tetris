use super::board::Board;
use super::piece::Piece;
use super::piece_type_bag_generator::PieceTypeGenerator;

use std::collections::VecDeque;
use web_sys::console;

pub struct Game {
    board: Board,
    piece: Option<Piece>,
    next_pieces_capacity: usize,
    next_pieces: VecDeque<Piece>,
    _shadow_piece: Option<Piece>,
    score: u32,
    generator: PieceTypeGenerator,
}

impl Game {
    pub fn new() -> Self {
        let next_pieces_capacity: usize = 3;
        Game{
            board: Board::new(20, 10),
            piece: None,
            next_pieces_capacity,
            next_pieces: VecDeque::with_capacity(next_pieces_capacity),
            _shadow_piece: None,
            score: 0,
            generator: PieceTypeGenerator::new(),
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
        if !self.board.is_colliding(&piece) {
            console::log_1(&"no collision".into());
            return
        }
        console::log_1(&"collision".into());
        piece.revert_move_down();
        if !self.board.is_fully_in(&piece) {
            self.game_over();
            return
        }
        self.board.freeze(self.piece.take().unwrap());
        console::log_1(&"froze to board".into());
        let cleared_lines = self.board.clear_lines();
        self.update_score(cleared_lines.len() as u32);
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
        if !self.board.is_colliding(&piece) {
            console::log_1(&"no collision".into());
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
        if !self.board.is_colliding(&piece) {
            console::log_1(&"no collision".into());
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
        if !self.board.is_colliding(&piece) {
            console::log_1(&"no collision".into());
            return
        }
        piece.rotate_anticlockwise();
    }

    pub fn run(&mut self) {
        console::log_1(&"Run".into());
        self.spawn();
    }

    fn create_next_piece(&mut self) -> Piece {
        Piece::new(0, 0, self.generator.next_piece_type())
    }

    fn fill_next_pieces(&mut self) {
        for _ in 0..(self.next_pieces_capacity - self.next_pieces.len()) {
            console::log_1(&"push new piece".into());
            let piece = self.create_next_piece();
            self.next_pieces.push_back(piece);
        }
    }

    fn pop_next_piece(&mut self) -> Piece {
        if self.next_pieces.len() < self.next_pieces_capacity {
            self.fill_next_pieces();
        }

        let piece = match self.next_pieces.pop_front() {
            Some(piece) => piece,
            None => self.create_next_piece(),
        };

        self.fill_next_pieces();

        piece
    }

    fn spawn(&mut self) {
        let piece = self.pop_next_piece();
        let row = self.board.height() as u32 - piece.empty_row_offset();
        let column = self.board.width() as u32 / 2 - piece.horizontal_center_offset();
        self.piece = Some(Piece::new(row as i32, column as i32, piece.piece_type()));
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

    pub fn next_pieces(&self) -> &VecDeque<Piece> {
        &self.next_pieces
    }
}
