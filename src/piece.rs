use std::ops::Add;

#[derive(Clone, Copy)]
pub enum Type {
    Bar,
//    T,
//    Square,
//    L,
//    ReverseL
//    S,
//    ReverseS
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Orientation {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Add<i8> for Orientation {
    type Output = Self;
    fn add(self, other: i8) -> Orientation {
        let mut result = (self as i8 + other) % 4;
        if result < 0 {
            result = result + 4;
        }
        match result {
            0 => Orientation::Top,
            1 => Orientation::Right,
            2 => Orientation::Bottom,
            3 => Orientation::Left,
            _ => panic!("result should be between 0 and 4"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell(pub Option<Type>);

pub type Shape = Vec<Vec<Cell>>;

pub trait Piece {
    fn row_offset(&self) -> i32;
    fn column_offset(&self) -> i32;
//    fn orientation(&self) -> Orientation;
    fn rotate_clockwise(&mut self);
    fn rotate_anticlockwise(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn revert_move_down(&mut self);
    fn shape(&self) -> &Shape;
}

trait PieceTemplate {
    fn shape(&self, orientation: Orientation) -> &Shape;
}

fn rotate_quarter_cycle_clockwise(shape: &Shape) -> Shape {
    let size = shape.len();
    for row in shape.iter() {
        if row.len() != size {
            panic!("This can only work on square matrix");
        }
    }
    let mut rotated = shape.clone();

    for (i, row) in shape.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            rotated[j][size-i-1] = *cell;
        }
    }

    rotated
}

pub fn index(shape_index: usize, offset: i32) -> Option<usize> {
    match offset {
        _ if offset < 0 => shape_index.checked_sub((-offset) as usize),
        _ if offset >= 0 => shape_index.checked_add(offset as usize),
        _ => panic!("Neither strictly negative nor positive ??"),
    }
}

// Bar Piece
struct BarTemplate {
    top_shape: Shape,
    right_shape: Shape,
    bottom_shape: Shape,
    left_shape: Shape,
}
impl BarTemplate {
    pub fn new() -> Self {
        let top_shape = Self::top_shape();
        let right_shape = rotate_quarter_cycle_clockwise(&Self::top_shape());
        let bottom_shape = rotate_quarter_cycle_clockwise(&right_shape);
        let left_shape = rotate_quarter_cycle_clockwise(&bottom_shape);

        Self {
            top_shape,
            right_shape,
            bottom_shape,
            left_shape,
        }
    }

    fn top_shape() -> Shape {
        vec![
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::Some(Type::Bar)), Cell(Option::Some(Type::Bar)), Cell(Option::Some(Type::Bar)), Cell(Option::Some(Type::Bar))],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None), Cell(Option::None)],
        ]
    }
}
impl PieceTemplate for BarTemplate {
    fn shape(&self, orientation: Orientation) -> &Shape {
        match orientation {
            Orientation::Top => &self.top_shape,
            Orientation::Right => &self.right_shape,
            Orientation::Bottom => &self.bottom_shape,
            Orientation::Left => &self.left_shape,
        }
    }
}

pub struct Bar {
    template: BarTemplate,
    row_offset: i32,
    column_offset: i32,
    orientation: Orientation,
}
impl Bar {
    pub fn new(row_offset: i32, column_offset: i32) -> Self {
        Self {
            template: BarTemplate::new(),
            row_offset,
            column_offset,
            orientation: Orientation::Top,
        }
    }
}
impl Piece for Bar {
    fn row_offset(&self) -> i32 {
        self.row_offset
    }
    fn column_offset(&self) -> i32 {
        self.column_offset
    }
//    fn orientation(&self) -> Orientation {
//        self.orientation
//    }
    fn rotate_clockwise(&mut self) {
        self.orientation = self.orientation + 1;
    }
    fn rotate_anticlockwise(&mut self) {
        self.orientation = self.orientation + -1;
    }
    fn move_down(&mut self) {
        self.row_offset -= 1;
    }
    fn move_left(&mut self) {
        self.column_offset -= 1;
    }
    fn move_right(&mut self) {
        self.column_offset += 1;
    }
    fn revert_move_down(&mut self) {
        self.row_offset += 1;
    }
    fn shape(&self) -> &Shape {
        self.template.shape(self.orientation)
    }
}
