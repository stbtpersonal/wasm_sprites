use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

use crate::canvas::Canvas;

pub struct Texture {
    width: f32,
    height: f32,
    texture: WebGlTexture,
}

impl Texture {
    pub fn new(canvas: &Canvas, image_id: &str) -> Texture {
        let image = Texture::get_image(image_id);
        let texture = Texture::initialize_texture(canvas, &image);
        Texture {
            width: image.width() as f32,
            height: image.height() as f32,
            texture,
        }
    }

    fn get_image(image_id: &str) -> HtmlImageElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let image = document.get_element_by_id(image_id).unwrap();
        image.dyn_into::<HtmlImageElement>().unwrap()
    }

    fn initialize_texture(canvas: &Canvas, image: &HtmlImageElement) -> WebGlTexture {
        let gl = canvas.gl();
        let texture = gl.create_texture().unwrap();
        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            0,
            web_sys::WebGlRenderingContext::RGBA as i32,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            image,
        )
        .unwrap();
        gl.generate_mipmap(web_sys::WebGlRenderingContext::TEXTURE_2D);
        texture
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }

    pub fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
