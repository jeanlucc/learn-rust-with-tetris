use super::piece;
use std::cell::Ref;

pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Vec<piece::Cell>>,
}

impl Board {
    pub fn new(height: u32, width: u32) -> Self {
        Board {
            width,
            height,
            cells: vec!(vec!(piece::Cell(Option::None); width as usize); height as usize),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> &Vec<Vec<piece::Cell>> {
        &self.cells
    }

    pub fn freeze(&mut self, piece: Ref<dyn piece::Piece>) {
        let shape = piece.shape();
        for (shape_row_index, row) in shape.iter().enumerate() {
            for (shape_column_index, cell) in row.iter().enumerate() {
                if let None = cell.0 {
                    continue;
                };

                let i = self.i(shape_row_index, piece.row_offset());
                let j = self.j(shape_column_index, piece.column_offset());

                self.cells[i][j] = *cell;
            }
        }
    }

    pub fn is_legal(&self, piece: Ref<dyn piece::Piece>) -> bool {
        let shape = piece.shape();
        for (shape_row_index, row) in shape.iter().enumerate() {
            for (shape_column_index, cell) in row.iter().enumerate() {
                if let None = cell.0 {
                    continue;
                };

                let i = piece::index(shape_row_index, piece.row_offset());
                let j = piece::index(shape_column_index, piece.column_offset());

                if None == i || i.unwrap() as u32 >= self.height || None == j || j.unwrap() as u32 >= self.width {
                    return false
                }

                if let Option::Some(_) = self.cells[i.unwrap()][j.unwrap()].0 {
                    return false
                }
            }
        }

        true
    }

    fn i(&self, shape_row_index: usize, row_offset: i32) -> usize {
        Self::in_limit_index(shape_row_index, row_offset, self.height)
    }

    fn j(&self, shape_column_index: usize, column_offset: i32) -> usize {
        Self::in_limit_index(shape_column_index, column_offset, self.width)
    }

    fn in_limit_index(shape_index: usize, offset: i32, limit: u32) -> usize {
        let index = piece::index(shape_index, offset);

        if None == index || index.unwrap() as u32 >= limit {
            panic!("Froze piece out of board");
        }

        index.unwrap()
    }
}