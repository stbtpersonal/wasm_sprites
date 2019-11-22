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
}
