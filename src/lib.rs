mod utils;
mod board;
mod draw;
mod piece;
mod game;

extern crate web_sys;

use game::Game;
use draw::draw;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let document = document();

    let game = Rc::new(RefCell::new(game::Game::new()));
    {
        let game = Rc::clone(&game);
        bind_key(&document, " ", move || {game.borrow_mut().pause()})?;
    }
    {
        let game = Rc::clone(&game);
        bind_key(&document, "ArrowDown", move || {game.borrow_mut().move_down()})?;
    }
    {
        let game = Rc::clone(&game);
        bind_key(&document, "ArrowUp", move || {game.borrow_mut().rotate_clockwise()})?;
    }
    {
        let game = Rc::clone(&game);
        bind_key(&document, "ArrowLeft", move || {game.borrow_mut().move_left()})?;
    }
    {
        let game = Rc::clone(&game);
        bind_key(&document, "ArrowRight", move || {game.borrow_mut().move_right()})?;
    }
    Rc::clone(&game).borrow_mut().run();
    setup_draw(Rc::clone(&game));
    Ok(())
}

fn bind_key<F>(document: &Document, key_name: &'static str, action: F) -> Result<(), JsValue>
where
    F: Fn() + 'static
{
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if key_name != event.key() {
            return;
        }
        action();
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", &closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

fn setup_draw(game: Rc<RefCell<Game>>) {
    let context = document()
        .get_element_by_id("board").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();
    let draw_func = Rc::new(RefCell::new(None));
    let init_draw_func = Rc::clone(&draw_func);
    *init_draw_func.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw(&game.borrow(), &context, 30);
        request_animation_frame(draw_func.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(init_draw_func.borrow().as_ref().unwrap());
}
