use wasm_bindgen::JsCast;
use web_sys;
use web_sys::{
    HtmlCanvasElement,
    WebGlRenderingContext,
    WebGlProgram,
    WebGlShader,
};

pub struct Canvas {
    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
}

impl Canvas {
    pub fn initialize(canvas_id: &str) -> Canvas {
        let canvas = Canvas::get_canvas(canvas_id);
        let gl = Canvas::initialize_gl(&canvas);

        Canvas {
            canvas,
            gl,
        }
    }

    fn get_canvas(canvas_id: &str) -> HtmlCanvasElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        canvas.dyn_into::<HtmlCanvasElement>().unwrap()
    }

    fn initialize_gl(canvas: &HtmlCanvasElement) -> WebGlRenderingContext {
        let gl = canvas.get_context("webgl").unwrap().unwrap();
        gl.dyn_into::<WebGlRenderingContext>().unwrap()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        let width = self.canvas.width();
        let height = self.canvas.height();
        (width, height)
    }

    pub fn gl(&self) -> &WebGlRenderingContext {
        &self.gl
    }

    pub fn compile_program(&self, vertex_shader_source: &str, fragment_shader_source: &str) -> WebGlProgram {
        let vertex_shader = self.compile_vertex_shader(vertex_shader_source);
        let fragment_shader = self.compile_fragment_shader(fragment_shader_source);

        let program = self.gl.create_program().unwrap();
        self.gl.attach_shader(&program, &vertex_shader);
        self.gl.attach_shader(&program, &fragment_shader);
        self.gl.link_program(&program);

        let program_link_status = self.gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap();
        if !program_link_status {
            let error = self.gl.get_program_info_log(&program).unwrap();
            panic!("Program link failed: {}", error);
        }

        program
    }

    fn compile_vertex_shader(&self, source: &str) -> WebGlShader {
        self.compile_shader(WebGlRenderingContext::VERTEX_SHADER, source)
    }

    fn compile_fragment_shader(&self, source: &str) -> WebGlShader {
        self.compile_shader(WebGlRenderingContext::FRAGMENT_SHADER, source)
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> WebGlShader {
        let shader = self.gl.create_shader(shader_type).unwrap();
        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);

        let shader_compile_status = self.gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap();
        if !shader_compile_status {
            let error = self.gl.get_shader_info_log(&shader).unwrap();
            panic!("Shader compilation failed: {}", error)
        }

        shader
    }

    pub fn use_program(&self, program: &web_sys::WebGlProgram) {
        self.gl.use_program(Some(program));
    }

    pub fn get_uniform_location(&self, program: &web_sys::WebGlProgram, name: &str) -> web_sys::WebGlUniformLocation {
        self.gl.get_uniform_location(program, name).unwrap()
    }

    pub fn uniform2f(&self, uniform_location: &web_sys::WebGlUniformLocation, x: f32, y: f32) {
        self.gl.uniform2f(Some(&uniform_location), x, y);
    }

    pub fn get_attrib_location(&self, program: &web_sys::WebGlProgram, name: &str) -> i32 {
        self.gl.get_attrib_location(program, name)
    }

    pub fn draw_vertices(&self, attrib_location: i32, vertices: &[f32]) {
        let buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vertices_array = js_sys::Float32Array::view(&vertices);
            self.gl.buffer_data_with_array_buffer_view(
                web_sys::WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                web_sys::WebGlRenderingContext::DYNAMIC_DRAW,
            );
        }

        self.gl.enable_vertex_attrib_array(attrib_location as u32);
        self.gl.vertex_attrib_pointer_with_i32(
            attrib_location as u32,
            2,
            web_sys::WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        let icon = Canvas::get_icon();
        let texture = self.gl.create_texture().unwrap();
        self.gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE0);
        self.gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        self.gl.tex_image_2d_with_u32_and_u32_and_image(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            0,
            web_sys::WebGlRenderingContext::RGBA as i32,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            &icon,
        ).unwrap();
        self.gl.generate_mipmap(web_sys::WebGlRenderingContext::TEXTURE_2D);

        self.gl.clear_color(1f32, 1f32, 0f32, 1f32);
        self.gl.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);
        self.gl.color_mask(true, true, true, false);
        self.gl.enable(web_sys::WebGlRenderingContext::BLEND);
        self.gl.blend_func(web_sys::WebGlRenderingContext::SRC_ALPHA, web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
        self.gl.draw_arrays(web_sys::WebGlRenderingContext::POINTS, 0, (vertices.len() / 2) as i32);
    }

    fn get_icon() -> web_sys::HtmlImageElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let icon = document.get_element_by_id("icon").unwrap();
        icon.dyn_into::<web_sys::HtmlImageElement>().unwrap()
    }
}
