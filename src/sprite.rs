use js_sys::Float32Array;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::canvas::Canvas;
use crate::point::Point;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::vector2d::Vector2D;

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
        let shader = Shader::new(
            canvas,
            SPRITE_VERTEX_SHADER_SOURCE,
            SPRITE_FRAGMENT_SHADER_SOURCE,
        );

        SpriteShader { shader }
    }

    pub fn program(&self) -> &WebGlProgram {
        self.shader.program()
    }
}

pub struct Sprite {
    canvas: Rc<Canvas>,
    shader: Rc<SpriteShader>,
    texture: Rc<Texture>,
    position: Point,
    velocity: Vector2D,
}

impl Sprite {
    pub fn new(canvas: Rc<Canvas>, shader: Rc<SpriteShader>, texture: Rc<Texture>) -> Sprite {
        Sprite {
            canvas,
            shader,
            texture,
            position: Point { x: 0f32, y: 0f32 },
            velocity: Vector2D { x: 0f32, y: 0f32 },
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity.x = x;
        self.velocity.y = y;
    }

    pub fn update(&mut self, delta_time: f32) {
        let (canvas_width, canvas_height) = self.canvas.dimensions();
        let (canvas_width, canvas_height) = (canvas_width as f32, canvas_height as f32);
        let (texture_width, texture_height) = self.texture.dimensions();
        let (texture_width, texture_height) = (texture_width as f32, texture_height as f32);

        let mut delta_x = delta_time * self.velocity.x;
        if (self.position.x + delta_x + (texture_width / 2f32) > canvas_width)
            || (self.position.x + delta_x - (texture_width / 2f32) < 0f32) {
            delta_x *= -1f32;
            self.velocity.x *= -1f32;
        }
        self.position.x += delta_x;

        let mut delta_y = delta_time * self.velocity.y;
        if (self.position.y + delta_y + (texture_height / 2f32) > canvas_height)
            || (self.position.y + delta_y - (texture_height / 2f32) < 0f32) {
            delta_y *= -1f32;
            self.velocity.y *= -1f32;
        }
        self.position.y += delta_y;
    }

    pub fn draw(&self) {
        let gl = self.canvas.gl();

        let program = self.shader.program();
        gl.use_program(Some(program));

        let screen_size_uniform_location = gl.get_uniform_location(program, "screenSize").unwrap();
        let (width, height) = self.canvas.dimensions();
        gl.uniform2f(
            Some(&screen_size_uniform_location),
            width as f32,
            height as f32,
        );

        let _texture = self.texture.texture();
        let sprite_texture_uniform_location =
            gl.get_uniform_location(program, "spriteTexture").unwrap();
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

        let sprite_position_attrib_location =
            gl.get_attrib_location(program, "spritePosition") as u32;
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
        gl.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);
        gl.color_mask(true, true, true, true);
    }
}
