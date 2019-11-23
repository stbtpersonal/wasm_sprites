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

#[wasm_bindgen(start)]
pub fn main() {
    let canvas = Canvas::initialize("canvas");
    let sprite_shader = SpriteShader::new(&canvas);
    let pikachu_texture = Texture::new(&canvas, "pikachu");
    let mut pikachu_sprite = Sprite::new(&canvas, &sprite_shader, &pikachu_texture);

    canvas.clear();
    pikachu_sprite.set_position(100f32, 250f32);
    pikachu_sprite.draw();

    console_log!("Success!");
}
