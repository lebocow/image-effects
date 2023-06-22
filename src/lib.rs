use base64::{
    engine::{self, general_purpose},
    Engine as _,
};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale scalled".into());

    let base64_to_vector: Vec<u8> = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let img: image::DynamicImage = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image Loaded".into());

    let new_img = img.grayscale();

    log(&"Grayscale effect applied".into());

    let mut buffer: Vec<u8> = Vec::new();
    new_img
        .write_to(&mut Cursor::new(&mut buffer), Png)
        .unwrap();
    log(&"New image written".into());

    let encoded_img: String = general_purpose::STANDARD.encode(&buffer);
    let data_url: String = format!("data:image/pnl;base64,{}", encoded_img);

    return data_url;
}
