mod gl_context;
pub use gl_context::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGlFramebuffer, WebGlRenderingContext, WebGlTexture};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn color_255_to_f32(data: &[u8]) -> Vec<f32> {
    let result = data.iter().map(|v| (*v as f32 / 255.)).collect();
    result
}

pub fn color_f32_to_255(data: &[f32]) -> Vec<u8> {
    let result = data.iter().map(|v| (v * 255.) as u8).collect();
    result
}

pub fn init_frame_buffer(
    gl: &WebGlRenderingContext,
    offscreen_width: usize,
    offsrceen_height: usize,
) -> Result<(WebGlFramebuffer, WebGlTexture), JsValue> {
    let frame_buffer = gl.create_framebuffer();
    let texture = gl.create_texture();
    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGBA as i32,
        offscreen_width as i32,
        offsrceen_height as i32,
        0,
        WebGlRenderingContext::RGBA,
        WebGlRenderingContext::UNSIGNED_BYTE,
        None,
    )?;
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );

    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );

    let depth_buffer = gl.create_renderbuffer();
    gl.bind_renderbuffer(WebGlRenderingContext::RENDERBUFFER, depth_buffer.as_ref());
    gl.renderbuffer_storage(
        WebGlRenderingContext::RENDERBUFFER,
        WebGlRenderingContext::DEPTH_COMPONENT16,
        offscreen_width as i32,
        offsrceen_height as i32,
    );

    gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, frame_buffer.as_ref());
    gl.framebuffer_texture_2d(
        WebGlRenderingContext::FRAMEBUFFER,
        WebGlRenderingContext::COLOR_ATTACHMENT0,
        WebGlRenderingContext::TEXTURE_2D,
        texture.as_ref(),
        0,
    );
    gl.framebuffer_renderbuffer(
        WebGlRenderingContext::RENDERBUFFER,
        WebGlRenderingContext::DEPTH_ATTACHMENT,
        WebGlRenderingContext::RENDERBUFFER,
        depth_buffer.as_ref(),
    );
    
    gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
    gl.bind_renderbuffer(WebGlRenderingContext::RENDERBUFFER, None);
    Ok((frame_buffer.unwrap(), texture.unwrap()))
}
