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

#[macro_use]
mod logging;

#[wasm_bindgen]
struct Game {
    canvas: Rc<Canvas>,
    sprite_shader: Rc<SpriteShader>,
    pikachu_texture: Rc<Texture>,
    pikachus: Vec<Sprite>,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let canvas = Rc::new(Canvas::initialize("canvas"));
        let sprite_shader = Rc::new(SpriteShader::new(&canvas));
        let pikachu_texture = Rc::new(Texture::new(&canvas, "pikachu"));

        let mut pikachu = Sprite::new(
            canvas.clone(),
            sprite_shader.clone(),
            pikachu_texture.clone(),
        );
        pikachu.set_position(100f32, 250f32);
        let pikachus = vec![pikachu];

        Game {
            canvas,
            sprite_shader,
            pikachu_texture,
            pikachus,
        }
    }

    pub fn tick(&self) {
        console_log!("Tick!");
    }
}