use super::piece;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct PieceTypeGenerator {
    bag: Vec<piece::Type>,
}

impl PieceTypeGenerator {
    pub fn new() -> Self {
        PieceTypeGenerator {
            bag: Vec::with_capacity(7),
        }
    }

    pub fn next_piece_type(&mut self) -> piece::Type {
        match self.bag.pop() {
            Some(piece_type) => piece_type,
            None => {
                self.bag = create_next_random_bag();
                self.bag.pop().unwrap()
            },
        }
    }
}

fn create_next_random_bag() -> Vec<piece::Type> {
    let mut rng = thread_rng();
    let mut bag = vec!(
        piece::Type::I,
        piece::Type::T,
        piece::Type::O,
        piece::Type::L,
        piece::Type::J,
        piece::Type::S,
        piece::Type::Z,
    );
    bag.shuffle(&mut rng);
    bag
}
