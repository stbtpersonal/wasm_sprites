use std::rc::Rc;

use wasm_bindgen::prelude::*;

use canvas::Canvas;
use sprite::Sprite;
use sprite::SpriteShader;
use texture::Texture;

#[macro_use]
mod logging;

mod canvas;
mod texture;
mod shader;
mod sprite;
mod point;
mod vector2d;

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
        pikachu.set_velocity(0.1f32, 0.1f32);
        let pikachus = vec![pikachu];

        Game {
            canvas,
            _sprite_shader: sprite_shader,
            _pikachu_texture: pikachu_texture,
            pikachus,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for pikachu in self.pikachus.iter_mut() {
            pikachu.update(delta_time);
        }
    }

    pub fn draw(&mut self) {
        self.canvas.clear();
        for pikachu in self.pikachus.iter() {
            pikachu.draw();
        }
    }
}