use crate::utils::{color_255_to_f32, color_f32_to_255, set_panic_hook, GlContext};
// use anyhow::Result;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsValue};

use web_sys::{ImageData, WebGlRenderingContext};

use super::vertices_attribute;

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

#[wasm_bindgen(js_name=gpuGrayScale)]
pub fn gpu_gray_scale(image_data: &ImageData, gl: &WebGlRenderingContext) -> Result<(), JsValue> {
    let mut gl_context = GlContext::new(gl);
    gl_context.compile_shader(
        include_str!("./gray.fragment.glsl"),
        WebGlRenderingContext::FRAGMENT_SHADER,
    )?;
    gl_context.compile_shader(
        include_str!("./gray.vertex.glsl"),
        WebGlRenderingContext::VERTEX_SHADER,
    )?;
    gl_context.link_program()?;
    gl.use_program(gl_context.program.as_ref());

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    let mut file_size = 0;
    unsafe {
        let vert_attribute_array = js_sys::Float32Array::view(&vertices_attribute);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_attribute_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        file_size = 4;
    }
    let a_position = gl.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_Position");
    gl.vertex_attrib_pointer_with_i32(
        a_position as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        (file_size * 4) as i32,
        0,
    );
    gl.enable_vertex_attrib_array(a_position as u32);

    let a_tex_coord = gl.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_TexCoord");
    gl.vertex_attrib_pointer_with_i32(
        a_tex_coord as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        (file_size * 4) as i32,
        file_size * 2,
    );
    gl.enable_vertex_attrib_array(a_tex_coord as u32);

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    let texture = gl
        .create_texture()
        .ok_or("failed create texture".to_string())?;
    let u_sampler = gl.get_uniform_location(gl_context.program.as_ref().unwrap(), "u_Sampler");
    gl.pixel_storei(WebGlRenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
    gl.active_texture(WebGlRenderingContext::TEXTURE0);
    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );

    gl.tex_image_2d_with_u32_and_u32_and_image_data(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGBA as i32,
        WebGlRenderingContext::RGBA,
        WebGlRenderingContext::UNSIGNED_BYTE,
        image_data,
    )?;
    gl.uniform1i(u_sampler.as_ref(), 0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
  
    Ok(())
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
