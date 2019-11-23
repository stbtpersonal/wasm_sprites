use wasm_bindgen::prelude::*;

mod canvas;
mod texture;
mod shader;
mod logging;

use canvas::Canvas;
use texture::Texture;
use shader::Shader;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let canvas = Canvas::initialize("canvas");
    let icon_texture = Texture::new(&canvas, "icon");

    let vertex_shader_source = "
        uniform vec2 screenSize;        // width/height of screen
        attribute vec2 spritePosition;  // position of sprite

        void main() {
            vec4 screenTransform = vec4(2.0 / screenSize.x, -2.0 / screenSize.y, -1.0, 1.0);
            gl_Position = vec4(spritePosition * screenTransform.xy + screenTransform.zw, 0.0, 1.0);
            gl_PointSize = 64.0;
        }
    ";
    let fragment_shader_source = "
        uniform sampler2D spriteTexture;  // texture we are drawing

        void main() {
            gl_FragColor = texture2D(spriteTexture, gl_PointCoord);
        }
    ";
    let shader_program = Shader::new(&canvas, vertex_shader_source, fragment_shader_source);
    let program = shader_program.program();

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
