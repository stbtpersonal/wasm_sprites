use std::rc::Rc;

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
pub struct Game {
    canvas: Rc<Canvas>,
    _sprite_shader: Rc<SpriteShader>,
    _pikachu_texture: Rc<Texture>,
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
            _sprite_shader: sprite_shader,
            _pikachu_texture: pikachu_texture,
            pikachus,
        }
    }

    pub fn tick(&mut self) {
        console_log!("Tick!");
        self.canvas.clear();
        for pikachu in self.pikachus.iter() {
            pikachu.draw();
        }
    }
}