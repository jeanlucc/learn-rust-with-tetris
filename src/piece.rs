use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::ops::Add;

#[derive(Clone, Copy)]
pub enum Type {
    I,
    T,
    O,
    L,
    J,
    S,
    Z,
}
impl Distribution<Type> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type {
        match rng.gen_range(0..7) {
            0 => Type::I,
            1 => Type::T,
            2 => Type::O,
            3 => Type::L,
            4 => Type::J,
            5 => Type::S,
            _ => Type::Z,
        }
    }
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

struct PieceTemplate {
    top_shape: Shape,
    right_shape: Shape,
    bottom_shape: Shape,
    left_shape: Shape,
}
impl PieceTemplate {
    pub fn new(piece_type: Type) -> Self {
        let top_shape = Self::type_top_shape(piece_type);
        let right_shape = rotate_quarter_cycle_clockwise(&top_shape);
        let bottom_shape = rotate_quarter_cycle_clockwise(&right_shape);
        let left_shape = rotate_quarter_cycle_clockwise(&bottom_shape);

        Self {
            top_shape,
            right_shape,
            bottom_shape,
            left_shape,
        }
    }

    fn type_top_shape(piece_type: Type) -> Shape {
        match piece_type {
            Type::I => Self::i_top_shape(),
            Type::T => Self::t_top_shape(),
            Type::O => Self::o_top_shape(),
            Type::L => Self::l_top_shape(),
            Type::J => Self::j_top_shape(),
            Type::S => Self::s_top_shape(),
            Type::Z => Self::z_top_shape(),
        }
    }

    fn top_shape(&self) -> &Shape {
        &self.top_shape
    }
    fn right_shape(&self) -> &Shape {
        &self.right_shape
    }
    fn bottom_shape(&self) -> &Shape {
        &self.bottom_shape
    }
    fn left_shape(&self) -> &Shape {
        &self.left_shape
    }

    fn i_top_shape() -> Shape {
        vec![
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::Some(Type::I)), Cell(Option::Some(Type::I)), Cell(Option::Some(Type::I)), Cell(Option::Some(Type::I))],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None), Cell(Option::None)],
        ]
    }
    fn t_top_shape() -> Shape {
        vec![
            vec![Cell(Option::None), Cell(Option::Some(Type::T)), Cell(Option::None)],
            vec![Cell(Option::Some(Type::T)), Cell(Option::Some(Type::T)), Cell(Option::Some(Type::T))],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None)],
        ]
    }
    fn o_top_shape() -> Shape {
        vec![
            vec![Cell(Option::Some(Type::O)), Cell(Option::Some(Type::O))],
            vec![Cell(Option::Some(Type::O)), Cell(Option::Some(Type::O))],
        ]
    }
    fn l_top_shape() -> Shape {
        vec![
            vec![Cell(Option::Some(Type::L)), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::Some(Type::L)), Cell(Option::Some(Type::L)), Cell(Option::Some(Type::L))],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None)],
        ]
    }
    fn j_top_shape() -> Shape {
        vec![
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::Some(Type::J))],
            vec![Cell(Option::Some(Type::J)), Cell(Option::Some(Type::J)), Cell(Option::Some(Type::J))],
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None)],
        ]
    }
    fn s_top_shape() -> Shape {
        vec![
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::Some(Type::S)), Cell(Option::Some(Type::S)), Cell(Option::None)],
            vec![Cell(Option::None), Cell(Option::Some(Type::S)), Cell(Option::Some(Type::S))],
        ]
    }
    fn z_top_shape() -> Shape {
        vec![
            vec![Cell(Option::None), Cell(Option::None), Cell(Option::None)],
            vec![Cell(Option::None), Cell(Option::Some(Type::Z)), Cell(Option::Some(Type::Z))],
            vec![Cell(Option::Some(Type::Z)), Cell(Option::Some(Type::Z)), Cell(Option::None)],
        ]
    }
}

pub struct Piece {
    template: PieceTemplate,
    row_offset: i32,
    column_offset: i32,
    orientation: Orientation,
}
impl Piece {
    pub fn new(row_offset: i32, column_offset: i32, piece_type: Type) -> Self {
        Self {
            template: PieceTemplate::new(piece_type),
            row_offset,
            column_offset,
            orientation: Orientation::Top,
        }
    }
    pub fn row_offset(&self) -> i32 {
        self.row_offset
    }
    pub fn column_offset(&self) -> i32 {
        self.column_offset
    }
    pub fn rotate_clockwise(&mut self) {
        self.orientation = self.orientation + 1;
    }
    pub fn rotate_anticlockwise(&mut self) {
        self.orientation = self.orientation + -1;
    }
    pub fn move_down(&mut self) {
        self.row_offset -= 1;
    }
    pub fn move_left(&mut self) {
        self.column_offset -= 1;
    }
    pub fn move_right(&mut self) {
        self.column_offset += 1;
    }
    pub fn revert_move_down(&mut self) {
        self.row_offset += 1;
    }
    pub fn shape(&self) -> &Shape {
        match self.orientation {
            Orientation::Top => &self.template.top_shape(),
            Orientation::Right => &self.template.right_shape(),
            Orientation::Bottom => &self.template.bottom_shape(),
            Orientation::Left => &self.template.left_shape(),
        }
    }
}
