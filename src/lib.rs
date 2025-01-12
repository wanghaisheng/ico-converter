use wasm_bindgen::prelude::*;
use image::{ImageBuffer, Rgba, ImageFormat};
use std::io::Cursor;

#[wasm_bindgen]
pub fn image_to_ico(image_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, JsValue> {
     let img = match image::load_from_memory(image_data){
        Ok(img) => img.resize(width, height, image::imageops::FilterType::Triangle),
        Err(_) => return Err(JsValue::from_str("Invalid image format"))
     };

    // convert the image to dynamic image
    let dynamic_img = img.to_rgba8();
    // convert dynamic image to image buffer
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, dynamic_img.into_raw()).unwrap();
  // Create a buffer for storing ico data
    let mut ico_data = Vec::new();
    match image::codecs::ico::IcoEncoder::new(&mut ico_data).encode(buffer.as_raw(), width, height, image::ColorType::Rgba8){
     Ok(_) => Ok(ico_data),
     Err(_) =>  return Err(JsValue::from_str("Failed to encode ico"))
    }
}
