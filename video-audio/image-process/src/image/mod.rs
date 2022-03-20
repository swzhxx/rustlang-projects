use crate::utils::{color_255_to_f32, color_f32_to_255, set_panic_hook};
// use anyhow::Result;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsValue};

use web_sys::{ImageData, WebGlRenderingContext};

#[wasm_bindgen(js_name = cpuGrayScale)]
/// 在Debug模式下，1080p的图片需要执行4s左右
pub fn cpu_gray_scale(image_data: &ImageData) -> Result<ImageData, JsValue> {
    set_panic_hook();
    // let height = image_data.height();
    let width = image_data.width();
    let data = image_data.data();
    let data = color_255_to_f32(&data);
    let mut i = 0;
    let mut gray_data = vec![];
    loop {
        let r = data[i];
        let g = data[i + 1];
        let b = data[i + 2];
        let a = data[i + 3];

        let gray = r * 0.3 + g * 0.59 + b * 0.11;
        gray_data.push(gray);
        gray_data.push(gray);
        gray_data.push(gray);
        gray_data.push(a);
        i += 4;
        if i >= data.len() {
            break;
        }
    }

    let gray_data = color_f32_to_255(&gray_data[..]);
    ImageData::new_with_u8_clamped_array(Clamped(&gray_data[..]), width)
}

#[wasm_bindgen(js_name=GpuGrayScale)]
pub fn gpu_gray_scale(image_data: &ImageData) -> Result<ImageData, JsValue> {
    todo!()
}

// #[cfg(test)]
// mod test {
//     use wasm_bindgen::Clamped;
//     use wasm_bindgen_test::wasm_bindgen_test;
//     use web_sys::ImageData;

//     use super::cpu_gray_scale;

//     #[wasm_bindgen_test]
//     fn test_cpu_gray_scale() {
//         let data: Vec<u8> = vec![255, 255, 0, 255];
//         let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), 1, 1).unwrap();
//         let gray_image_data = cpu_gray_scale(&image_data).unwrap();
//         println!("{:?}", gray_image_data.data())
//     }
// }
