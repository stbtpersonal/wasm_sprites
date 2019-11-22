use wasm_bindgen::prelude::*;

mod canvas;
mod logging;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let canvas = canvas::Canvas::initialize();
    let (width, height) = canvas.dimensions();
    console_log!("Width: {}, Height: {}", width, height);

    let vertex_shader = canvas.compile_vertex_shader("
        uniform vec2 screenSize;        // width/height of screen
        attribute vec2 spritePosition;  // position of sprite

        void main() {
            vec4 screenTransform = vec4(2.0 / screenSize.x, -2.0 / screenSize.y, -1.0, 1.0);
            gl_Position = vec4(spritePosition * screenTransform.xy + screenTransform.zw, 0.0, 1.0);
            gl_PointSize = 64.0;
        }
    ")?;
    let fragment_shader = canvas.compile_fragment_shader("
        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    ")?;
    let program = canvas.compile_program(&vertex_shader, &fragment_shader)?;
    console_log!("Program: {:?}", program);

    Ok(())
}
