use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use canvas::Canvas;
use sprite::Sprite;
use sprite::SpriteShader;
use texture::Texture;

mod canvas;
mod texture;
mod shader;
mod sprite;
mod point;
mod logging;

struct State {
    canvas: Canvas,
    pikachus: Vec<Sprite<'static>>,
}

#[wasm_bindgen(start)]
pub fn main() {
    let state = initialize_state();
    run(state);

    console_log!("Success!");
}

fn initialize_state() -> State {
    let canvas = Canvas::initialize("canvas");
    let sprite_shader = SpriteShader::new(&canvas);
    let pikachu_texture = Texture::new(&canvas, "pikachu");

    let mut pikachus = Vec::new();
    let mut pikachu = Sprite::new(&canvas, &sprite_shader, &pikachu_texture);
    pikachu.set_position(100f32, 250f32);
//    pikachus.push(pikachu);

    State {
        canvas,
        pikachus,
    }
}

fn run(state: State) {
    let tick_ref = Rc::new(RefCell::new(None));
    let tick_ref_clone = tick_ref.clone();

    *tick_ref_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        console_log!("Tick!");
        state.canvas.clear();
        request_animation_frame(tick_ref.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(tick_ref_clone.borrow().as_ref().unwrap());
}

fn request_animation_frame(func: &Closure<dyn FnMut()>) {
    let window = web_sys::window().unwrap();
    window.request_animation_frame(func.as_ref().unchecked_ref()).unwrap();
}
