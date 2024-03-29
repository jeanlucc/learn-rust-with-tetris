use super::piece::Piece;
use super::piece;

pub struct Board {
    width: usize,
    height: usize,
    max_piece_size: usize,
    cells: Vec<Vec<piece::Cell>>,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let max_piece_size = 4;
        Board {
            width,
            height,
            max_piece_size,
            cells: vec!(vec!(piece::Cell(Option::None); width); height + max_piece_size),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn max_piece_size(&self) -> usize {
        self.max_piece_size
    }

    pub fn cells(&self) -> &Vec<Vec<piece::Cell>> {
        &self.cells
    }

    pub fn freeze(&mut self, piece: Piece) {
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

    pub fn is_colliding(&self, piece: &Piece) -> bool {
        !self.in_available_cells_below(piece, self.height_with_hidden_top())
    }

    pub fn is_fully_in(&self, piece: &Piece) -> bool {
        self.in_available_cells_below(piece, self.height as usize)
    }

    pub fn clear_lines(&mut self) -> Vec<usize> {
        let mut index_to_remove = Vec::with_capacity(self.max_piece_size as usize);
        for (row_index, row) in self.cells().iter().enumerate() {
            let mut is_index_to_remove = true;
            for cell in row.iter() {
                if let None = cell.0 {
                    is_index_to_remove = false;
                    break;
                }
            }
            if is_index_to_remove {
                index_to_remove.push(row_index);
            }
        }
        for &index in index_to_remove.iter().rev() {
            self.cells.remove(index);
        }
        self.cells.resize(self.height_with_hidden_top(), vec!(piece::Cell(Option::None); self.width as usize));
        index_to_remove
    }

    fn height_with_hidden_top(&self) -> usize {
        self.height + self.max_piece_size
    }

    fn i(&self, shape_row_index: usize, row_offset: i32) -> usize {
        Self::in_limit_index(shape_row_index, row_offset, self.height)
    }

    fn j(&self, shape_column_index: usize, column_offset: i32) -> usize {
        Self::in_limit_index(shape_column_index, column_offset, self.width)
    }

    fn in_limit_index(shape_index: usize, offset: i32, limit: usize) -> usize {
        let index = piece::index(shape_index, offset);

        if None == index || index.unwrap() >= limit {
            panic!("Froze piece out of board");
        }

        index.unwrap()
    }

    fn in_available_cells_below(&self, piece: &Piece, height: usize) -> bool {
        let shape = piece.shape();
        for (shape_row_index, row) in shape.iter().enumerate() {
            for (shape_column_index, cell) in row.iter().enumerate() {
                if let None = cell.0 {
                    continue;
                };

                let i = piece::index(shape_row_index, piece.row_offset());
                let j = piece::index(shape_column_index, piece.column_offset());

                if None == i || i.unwrap() >= height || None == j || j.unwrap() >= self.width {
                    return false
                }

                if let Option::Some(_) = self.cells[i.unwrap()][j.unwrap()].0 {
                    return false
                }
            }
        }

        true
    }
}
