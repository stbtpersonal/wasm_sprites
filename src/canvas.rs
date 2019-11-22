use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
    gl_context: web_sys::WebGlRenderingContext,
}

impl Canvas {
    pub fn initialize() -> Canvas {
        let canvas = Canvas::get_canvas().unwrap();
        let gl_context = Canvas::get_gl_context(&canvas).unwrap();

        Canvas {
            canvas,
            gl_context,
        }
    }

    fn get_canvas() -> Result<web_sys::HtmlCanvasElement, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        Ok(canvas)
    }

    fn get_gl_context(canvas: &web_sys::HtmlCanvasElement) -> Result<web_sys::WebGlRenderingContext, JsValue> {
        let gl_context = canvas.get_context("webgl")?.unwrap();
        let gl_context = gl_context.dyn_into::<web_sys::WebGlRenderingContext>()?;
        Ok(gl_context)
    }

    pub fn dimensions(&self) -> (u32, u32) {
        let width = self.canvas.width();
        let height = self.canvas.height();
        (width, height)
    }

    pub fn compile_vertex_shader(&self, source: &str) -> Result<web_sys::WebGlShader, String> {
        self.compile_shader(web_sys::WebGlRenderingContext::VERTEX_SHADER, source)
    }

    pub fn compile_fragment_shader(&self, source: &str) -> Result<web_sys::WebGlShader, String> {
        self.compile_shader(web_sys::WebGlRenderingContext::FRAGMENT_SHADER, source)
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<web_sys::WebGlShader, String> {
        let shader = self.gl_context.create_shader(shader_type).unwrap();

        self.gl_context.shader_source(&shader, source);
        self.gl_context.compile_shader(&shader);

        let shader_compile_status = self.gl_context
            .get_shader_parameter(&shader, web_sys::WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap();
        if !shader_compile_status {
            return Err(self.gl_context.get_shader_info_log(&shader).unwrap());
        }

        Ok(shader)
    }

    pub fn compile_program(&self, vertex_shader: &web_sys::WebGlShader, fragment_shader: &web_sys::WebGlShader) -> Result<web_sys::WebGlProgram, String> {
        let program = self.gl_context.create_program().unwrap();
        self.gl_context.attach_shader(&program, vertex_shader);
        self.gl_context.attach_shader(&program, fragment_shader);
        self.gl_context.link_program(&program);

        let program_link_status = self.gl_context
            .get_program_parameter(&program, web_sys::WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap();
        if !program_link_status {
            return Err(self.gl_context.get_program_info_log(&program).unwrap());
        }

        Ok(program)
    }

    pub fn use_program(&self, program: &web_sys::WebGlProgram) {
        self.gl_context.use_program(Some(program));
    }

    pub fn get_uniform_location(&self, program: &web_sys::WebGlProgram, name: &str) -> web_sys::WebGlUniformLocation {
        self.gl_context.get_uniform_location(program, name).unwrap()
    }

    pub fn uniform2f(&self, uniform_location: &web_sys::WebGlUniformLocation, x: f32, y: f32) {
        self.gl_context.uniform2f(Some(&uniform_location), x, y);
    }

    pub fn get_attrib_location(&self, program: &web_sys::WebGlProgram, name: &str) -> i32 {
        self.gl_context.get_attrib_location(program, name)
    }

    pub fn draw_vertices(&self, attrib_location: i32, vertices: &[f32]) {
        let buffer = self.gl_context.create_buffer().unwrap();
        self.gl_context.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vertices_array = js_sys::Float32Array::view(&vertices);
            self.gl_context.buffer_data_with_array_buffer_view(
                web_sys::WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                web_sys::WebGlRenderingContext::DYNAMIC_DRAW,
            );
        }

        self.gl_context.enable_vertex_attrib_array(attrib_location as u32);
        self.gl_context.vertex_attrib_pointer_with_i32(
            attrib_location as u32,
            2,
            web_sys::WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        let icon = Canvas::get_icon();
        let texture = self.gl_context.create_texture().unwrap();
        self.gl_context.active_texture(web_sys::WebGlRenderingContext::TEXTURE0);
        self.gl_context.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        self.gl_context.tex_image_2d_with_u32_and_u32_and_image(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            0,
            web_sys::WebGlRenderingContext::RGBA as i32,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            &icon,
        ).expect("???");
        self.gl_context.generate_mipmap(web_sys::WebGlRenderingContext::TEXTURE_2D);

        self.gl_context.clear_color(1f32, 1f32, 0f32, 1f32);
        self.gl_context.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);
        self.gl_context.color_mask(true, true, true, false);
        self.gl_context.enable(web_sys::WebGlRenderingContext::BLEND);
        self.gl_context.blend_func(web_sys::WebGlRenderingContext::SRC_ALPHA, web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
        self.gl_context.draw_arrays(web_sys::WebGlRenderingContext::POINTS, 0, (vertices.len() / 2) as i32);
    }

    fn get_icon() -> web_sys::HtmlImageElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let icon = document.get_element_by_id("icon").unwrap();
        icon.dyn_into::<web_sys::HtmlImageElement>().unwrap()
    }
}
