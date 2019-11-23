use wasm_bindgen::prelude::*;

mod canvas;
mod texture;
mod shader;
mod sprite;
mod point;
mod logging;

use canvas::Canvas;
use texture::Texture;
use sprite::SpriteShader;
use sprite::Sprite;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let canvas = Canvas::initialize("canvas");
    let sprite_shader = SpriteShader::new(&canvas);
    let icon_texture = Texture::new(&canvas, "icon");
    let sprite = Sprite::new(&sprite_shader, &icon_texture);

    let program = sprite_shader.program();
    canvas.use_program(program);
    let screen_size_uniform_location = canvas.get_uniform_location(program, "screenSize");
    let (width, height) = canvas.dimensions();
    canvas.uniform2f(&screen_size_uniform_location, width as f32, height as f32);

    let sprite_position_attrib_location = canvas.get_attrib_location(program, "spritePosition");
    let vertices = [300f32, 400f32, 100f32, 100f32, 140f32, 80f32];
    canvas.draw_vertices(sprite_position_attrib_location, &vertices);

    console_log!("Success!");

    Ok(())
}
