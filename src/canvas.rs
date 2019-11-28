use wasm_bindgen::JsCast;
use web_sys;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub struct Canvas {
    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
}

impl Canvas {
    pub fn initialize(canvas_id: &str) -> Canvas {
        let canvas = Canvas::get_canvas(canvas_id);
        let gl = Canvas::initialize_gl(&canvas);

        Canvas { canvas, gl }
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

    pub fn dimensions(&self) -> (f32, f32) {
        let width = self.canvas.width() as f32;
        let height = self.canvas.height() as f32;
        (width, height)
    }

    pub fn gl(&self) -> &WebGlRenderingContext {
        &self.gl
    }

    pub fn clear(&self) {
        self.gl.clear_color(1f32, 1f32, 0f32, 1f32);
        self.gl
            .clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);
    }
}
