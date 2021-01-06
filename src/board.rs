use super::piece::Piece;
use super::piece;

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
            cells: vec!(vec!(piece::Cell(Option::None); width as usize); Self::compute_height_with_hidden_top(height)),
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
        let mut index_to_remove = Vec::with_capacity(4);
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

    fn compute_height_with_hidden_top(height: u32) -> usize {
        height as usize + 4
    }

    fn height_with_hidden_top(&self) -> usize {
        Self::compute_height_with_hidden_top(self.height)
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

    fn in_available_cells_below(&self, piece: &Piece, height: usize) -> bool {
        let shape = piece.shape();
        for (shape_row_index, row) in shape.iter().enumerate() {
            for (shape_column_index, cell) in row.iter().enumerate() {
                if let None = cell.0 {
                    continue;
                };

                let i = piece::index(shape_row_index, piece.row_offset());
                let j = piece::index(shape_column_index, piece.column_offset());

                if None == i || i.unwrap() >= height || None == j || j.unwrap() as u32 >= self.width {
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
