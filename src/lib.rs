use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue>  {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let p_element = document.create_element("p")?;
    p_element.set_inner_html("Hello world!");
    body.append_child(&p_element)?;

    Ok(())
}
