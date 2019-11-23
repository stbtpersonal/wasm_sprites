use web_sys::{
    WebGlProgram,
};

use crate::canvas::Canvas;
use crate::shader::Shader;

const SPRITE_VERTEX_SHADER_SOURCE: &str = "
        uniform vec2 screenSize;        // width/height of screen
        attribute vec2 spritePosition;  // position of sprite

        void main() {
            vec4 screenTransform = vec4(2.0 / screenSize.x, -2.0 / screenSize.y, -1.0, 1.0);
            gl_Position = vec4(spritePosition * screenTransform.xy + screenTransform.zw, 0.0, 1.0);
            gl_PointSize = 64.0;
        }
    ";

const SPRITE_FRAGMENT_SHADER_SOURCE: &str = "
        uniform sampler2D spriteTexture;  // texture we are drawing

        void main() {
            gl_FragColor = texture2D(spriteTexture, gl_PointCoord);
        }
    ";

pub struct SpriteShader {
    shader: Shader,
}

impl SpriteShader {
    pub fn new(canvas: &Canvas) -> SpriteShader {
        let shader = Shader::new(canvas, SPRITE_VERTEX_SHADER_SOURCE, SPRITE_FRAGMENT_SHADER_SOURCE);

        SpriteShader {
            shader,
        }
    }

    pub fn program(&self) -> &WebGlProgram {
        self.shader.program()
    }
}