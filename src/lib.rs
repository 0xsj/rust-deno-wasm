extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{ReadableStream, Request, Response, ResponseInit, Uint8Array};

#[wasm_bindgen]
pub fn handler(_request: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();

    // Create a ResponseInit object
    let response_init = ResponseInit::new()
        .status(200)
        .status_text("OK")
        .headers(&JsValue::from_str("Content-Type: text/plain"));

    // Create a body for the response
    let body = Uint8Array::new_with_length(13);
    body.fill(114); // Fill the array with ASCII code for 'r'

    // Create a ReadableStream from the Uint8Array
    let stream = ReadableStream::from(Uint8Array::from(body.as_ref()));

    // Set the body of the response using the set_body method
    response_init.set_body(Some(&stream));

    // Create a Response object with the ResponseInit object
    let response = Response::new_with_opt_init(&response_init)?;

    Ok(response)
}
