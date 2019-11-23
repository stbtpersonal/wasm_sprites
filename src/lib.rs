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
    let icon_texture = Texture::new(&canvas, "icon");
    let mut sprite = Sprite::new(&canvas, &sprite_shader, &icon_texture);

    canvas.clear();
    sprite.set_position(100f32, 250f32);
    sprite.draw();

    console_log!("Success!");
}
