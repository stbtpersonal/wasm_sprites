use js_sys::Float32Array;
use web_sys::{
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::canvas::Canvas;
use crate::point::Point;
use crate::shader::Shader;
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
    canvas: &'a Canvas,
    shader: &'a SpriteShader,
    texture: &'a Texture,
    position: Point,
}

impl<'a> Sprite<'a> {
    pub fn new(canvas: &'a Canvas, shader: &'a SpriteShader, texture: &'a Texture) -> Sprite<'a> {
        Sprite {
            canvas,
            shader,
            texture,
            position: Point { x: 0f32, y: 0f32 },
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn draw(&self) {
        let gl = self.canvas.gl();

        let program = self.shader.program();
        gl.use_program(Some(program));

        let screen_size_uniform_location = gl.get_uniform_location(program, "screenSize").unwrap();
        let (width, height) = self.canvas.dimensions();
        gl.uniform2f(Some(&screen_size_uniform_location), width as f32, height as f32);

        let _texture = self.texture.texture();
        let sprite_texture_uniform_location = gl.get_uniform_location(program, "spriteTexture").unwrap();
        gl.uniform1i(Some(&sprite_texture_uniform_location), 0);

        let vertices = [self.position.x, self.position.y];
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let vertices_array = Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGlRenderingContext::DYNAMIC_DRAW,
            );
        }

        let sprite_position_attrib_location = gl.get_attrib_location(program, "spritePosition") as u32;
        gl.enable_vertex_attrib_array(sprite_position_attrib_location);
        gl.vertex_attrib_pointer_with_i32(
            sprite_position_attrib_location,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        gl.color_mask(true, true, true, false);
        gl.enable(WebGlRenderingContext::BLEND);
        gl.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
        gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);
    }
}