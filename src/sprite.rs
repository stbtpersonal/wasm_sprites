use web_sys::WebGlProgram;

use crate::canvas::Canvas;
use crate::shader::Shader;
use crate::point::Point;
use crate::texture::Texture;

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

pub struct Sprite<'a> {
    shader: &'a SpriteShader,
    texture: &'a Texture,
    position: Point,
}

impl<'a> Sprite<'a> {
    pub fn new(shader: &'a SpriteShader, texture: &'a Texture) -> Sprite<'a> {
        Sprite {
            shader,
            texture,
            position: Point { x: 0, y: 0 },
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position.x = x;
        self.position.y = y;
    }
}