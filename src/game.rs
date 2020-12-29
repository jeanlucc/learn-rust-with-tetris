use super::board::Board;
use super::piece::Piece;
use super::piece;

use std::rc::Rc;
use std::cell::RefCell;
use web_sys::console;

pub struct Game {
    board: Board,
    piece: Option<Rc<RefCell<dyn Piece>>>,
    _next_piece: Option<Rc<RefCell<dyn Piece>>>,
    _shadow_piece: Option<Rc<RefCell<dyn Piece>>>,
}

impl Game {
    pub fn new() -> Self {
        Game{
            board: Board::new(20, 10),
            piece: None,
            _next_piece: None,
            _shadow_piece: None,
        }
    }

    pub fn pause(&mut self) {
        console::log_1(&"Pause".into());
    }

    pub fn move_down(&mut self) {
        console::log_1(&"move_down".into());
        let piece = match &self.piece {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.borrow_mut().move_down();
        if self.board.is_legal(piece.borrow()) {
            console::log_1(&"legal".into());
            return
        }
        piece.borrow_mut().revert_move_down();
        self.board.freeze(piece.borrow());
        self.spawn();
    }

    pub fn move_left(&mut self) {
        console::log_1(&"move_left".into());
        let piece = match self.piece.as_ref() {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.borrow_mut().move_left();
        if self.board.is_legal(piece.borrow()) {
            console::log_1(&"legal".into());
            return
        }
        piece.borrow_mut().move_right();
    }

    pub fn move_right(&mut self) {
        console::log_1(&"move_right".into());
        let piece = match &self.piece {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.borrow_mut().move_right();
        if self.board.is_legal(piece.borrow()) {
            console::log_1(&"legal".into());
            return
        }
        piece.borrow_mut().move_left();
    }

    pub fn rotate_clockwise(&mut self) {
        console::log_1(&"rotate_clockwise".into());
        let piece = match &self.piece {
            None => {
                console::log_1(&"no piece".into());
                return;
            },
            Some(piece) => piece,
        };
        piece.borrow_mut().rotate_clockwise();
        if self.board.is_legal(piece.borrow()) {
            console::log_1(&"legal".into());
            return
        }
        piece.borrow_mut().rotate_anticlockwise();
    }

    pub fn run(&mut self) {
        console::log_1(&"Run".into());
        self.spawn();
    }

    fn create_random_piece() -> Rc<RefCell<dyn Piece>> {
        let mut bar = piece::Bar::new(16,0);
        bar.rotate_clockwise();
        Rc::new(RefCell::new(bar))
    }

    fn spawn(&mut self) {
        let piece = Self::create_random_piece();
        if !self.board.is_legal(piece.borrow()) {
            self.game_over();
            return
        }
        self.piece = Some(piece);
        console::log_1(&"spawned".into());
    }

    fn game_over(&mut self) {
        console::log_1(&"game over".into());
    }
}

impl Game {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn piece(&self) -> Option<Rc<RefCell<dyn Piece>>> {
        match &self.piece {
            Some(piece) => Some(Rc::clone(&piece)),
            None => None,
        }
    }
}
