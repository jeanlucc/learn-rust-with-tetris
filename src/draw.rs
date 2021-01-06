use super::board::Board;
use super::game::Game;
use super::piece;
use super::piece::Piece;

use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;

struct DrawContext<'a> {
    canvas_context: &'a CanvasRenderingContext2d,
    zoom: u32,
}

impl<'a> DrawContext<'a> {
    pub fn new(canvas_context: &'a CanvasRenderingContext2d, zoom: u32) -> DrawContext {
        DrawContext {canvas_context, zoom}
    }
}

pub fn draw(game: &Game, context: &CanvasRenderingContext2d, zoom: u32) {
    let board = game.board();
    let canvas = context.canvas().unwrap();
    canvas.set_width(board.width() * (zoom+1));
    canvas.set_height((board.height()+4) * (zoom+1));
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    let context = DrawContext::new(&context, zoom);
    draw_grid(&context, board.width(), board.height());
    draw_board_cells(&context, board);
    if let Some(piece) = game.piece() {
        draw_piece(&context, &piece);
    }
}

fn draw_grid(context: &DrawContext, width: u32, height: u32) {
    let (context, zoom) = (context.canvas_context, context.zoom);
    context.set_stroke_style(&JsValue::from_str("#AAA"));
    context.begin_path();
    for row in 0..(height+1+4) {
        context.move_to(0., (row*(zoom+1)) as f64);
        context.line_to((width*(zoom+1)) as f64, (row*(zoom+1)) as f64);
    }
    for column in 0..width+1 {
        context.move_to((column*(zoom+1)) as f64, 0.);
        context.line_to((column*(zoom+1)) as f64, ((height+4)*(zoom+1)) as f64);
    }
    context.stroke();
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("#F00"));
    context.move_to(0., (4*(zoom+1)) as f64);
    context.line_to((width*(zoom+1)) as f64, (4*(zoom+1)) as f64);
    context.stroke();
}

fn draw_board_cells(context: &DrawContext, board: &Board) {
    for (row_index, row) in board.cells().iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            let color = get_cell_color(cell);
            draw_cell_with_color(context, row_index, column_index, &color);
        }
    }
}

fn draw_piece(context: &DrawContext, piece: &Piece) {
    let mut is_color_set = false;
    for (shape_row_index, row) in piece.shape().iter().enumerate() {
        for (shape_column_index, cell) in row.iter().enumerate() {
            if let None = cell.0 {
                continue;
            };

            let row = piece::index(shape_row_index, piece.row_offset());
            let column = piece::index(shape_column_index, piece.column_offset());
            let (row, column) = match (row, column) {
                (Some(row), Some(column)) => (row, column),
                _ => continue,
            };
            if !is_color_set {
                let color = get_cell_color(cell);
                context.canvas_context.set_fill_style(&JsValue::from_str(&color));
                is_color_set = true;
            }
            draw_cell(context, row, column);
        }
    }
}

fn draw_cell_with_color(context: &DrawContext, row: usize, column: usize, color: &str) {
    context.canvas_context.set_fill_style(&JsValue::from_str(color));
    draw_cell(context, row, column);
}

fn draw_cell(context: &DrawContext, row: usize, column: usize) {
    let DrawContext{canvas_context: context, zoom} = *context;
    context.fill_rect(
        (column * (zoom as usize + 1) + 1) as f64,
        (context.canvas().unwrap().height() as usize - (row + 1) * (zoom as usize + 1) + 1) as f64,
        zoom as f64,
        zoom as f64
    );
    context.stroke();
}

fn get_cell_color(cell: &piece::Cell) -> String {
    match cell.0 {
        Some(piece::Type::I) => "cyan".to_string(),
        Some(piece::Type::T) => "purple".to_string(),
        Some(piece::Type::O) => "yellow".to_string(),
        Some(piece::Type::L) => "orange".to_string(),
        Some(piece::Type::J) => "blue".to_string(),
        Some(piece::Type::S) => "lime".to_string(),
        Some(piece::Type::Z) => "red".to_string(),
        None => "#DDD".to_string(),
    }
}
