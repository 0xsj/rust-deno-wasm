extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{Request, Response, ResponseInit};

#[wasm_bindgen]
pub fn handler(_request: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();

    // Create a response with "rust says hi" as the body
    let response_init = ResponseInit::new()
        .status(200)
        .status_text("OK")
        .headers(&JsValue::from_str("Content-Type: text/plain"))
        .body(Some(&JsValue::from_str("rust says hi")));
    let response = Response::new_with_opt_init(&response_init)?;

    Ok(response)
}
