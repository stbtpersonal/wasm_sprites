use std::rc::Rc;

use js_sys::Math;
use wasm_bindgen::prelude::*;

use canvas::Canvas;
use sprite::Sprite;
use sprite::SpriteShader;
use texture::Texture;

#[macro_use]
mod logging;

mod canvas;
mod point;
mod shader;
mod sprite;
mod texture;
mod vector2d;

const PIKACHU_COUNT: i32 = 25;
const PIKACHU_MAX_VELOCITY: f32 = 0.5;

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

        let (canvas_width, canvas_height) = canvas.dimensions();
        let (pikachu_width, pikachu_height) = pikachu_texture.dimensions();

        let mut pikachus = Vec::new();
        for _ in 0..PIKACHU_COUNT {
            let mut pikachu = Sprite::new(
                canvas.clone(),
                sprite_shader.clone(),
                pikachu_texture.clone(),
            );

            let random_position_x =
                pikachu_width + ((canvas_width - pikachu_width) * Math::random() as f32);
            let random_position_y =
                pikachu_height + ((canvas_height - pikachu_height) * Math::random() as f32);
            pikachu.set_position(random_position_x, random_position_y);

            let random_velocity_x =
                ((Math::random() as f32) * PIKACHU_MAX_VELOCITY * 2f32) - PIKACHU_MAX_VELOCITY;
            let random_velocity_y =
                ((Math::random() as f32) * PIKACHU_MAX_VELOCITY * 2f32) - PIKACHU_MAX_VELOCITY;
            pikachu.set_velocity(random_velocity_x, random_velocity_y);

            pikachus.push(pikachu);
        }

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
