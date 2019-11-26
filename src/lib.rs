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
    canvas: Rc<Canvas>,
    _sprite_shader: Rc<SpriteShader>,
    _pikachu_texture: Rc<Texture>,
    pikachus: Rc<Vec<Sprite>>,
}

#[wasm_bindgen(start)]
pub fn main() {
    let state = initialize_state();
    run(state);

    console_log!("Success!");
}

fn initialize_state() -> State {
    let canvas = Rc::new(Canvas::initialize("canvas"));
    let sprite_shader = Rc::new(SpriteShader::new(&canvas));
    let pikachu_texture = Rc::new(Texture::new(&canvas, "pikachu"));

    let mut pikachu = Sprite::new(
        canvas.clone(),
        sprite_shader.clone(),
        pikachu_texture.clone(),
    );
    pikachu.set_position(100f32, 250f32);
    let pikachus = Rc::new(vec![pikachu]);

    State {
        canvas,
        _sprite_shader: sprite_shader,
        _pikachu_texture: pikachu_texture,
        pikachus,
    }
}

fn run(state: State) {
    let canvas = state.canvas;
    let pikachus = state.pikachus;

    let tick_ref = Rc::new(RefCell::new(None));
    let tick_ref_clone = tick_ref.clone();
    *tick_ref_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        console_log!("Tick!");

        canvas.clear();
        for pikachu in pikachus.iter() {
            pikachu.draw();
        }

        request_animation_frame(tick_ref.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(tick_ref_clone.borrow().as_ref().unwrap());
}

fn request_animation_frame(func: &Closure<dyn FnMut()>) {
    let window = web_sys::window().unwrap();
    window.request_animation_frame(func.as_ref().unchecked_ref()).unwrap();
}
