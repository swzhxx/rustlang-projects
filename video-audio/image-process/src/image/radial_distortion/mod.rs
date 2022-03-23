use super::vertices_attribute;
use crate::utils::GlContext;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{ImageData, WebGlRenderbuffer, WebGlRenderingContext};

#[wasm_bindgen(js_name=radialDistortion)]
pub fn radial_distortion(
    image_data: &ImageData,
    ctx: &WebGlRenderingContext,
    distortion_value: f32,
) -> Result<(), JsValue> {
    let mut gl_context = GlContext::new(ctx);
    gl_context.compile_shader(
        include_str!("./radial.vertex.glsl"),
        WebGlRenderingContext::VERTEX_SHADER,
    )?;

    gl_context.compile_shader(
        include_str!("./radial.fragment.glsl"),
        WebGlRenderingContext::FRAGMENT_SHADER,
    )?;
    gl_context.link_program()?;
    ctx.use_program(gl_context.program.as_ref());

    let buffer = ctx.create_buffer();
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, buffer.as_ref());
    unsafe {
        let vertex_attrib = js_sys::Float32Array::view(&vertices_attribute.clone());
        ctx.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_attrib,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    let a_position = ctx.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_Position");
    let file_size = 4;
    ctx.vertex_attrib_pointer_with_i32(
        a_position as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        file_size * 4,
        0,
    );
    ctx.enable_vertex_attrib_array(a_position as u32);

    let a_texcoord = ctx.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_TexCoord");
    ctx.vertex_attrib_pointer_with_i32(
        a_texcoord as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        file_size * 4,
        file_size * 2,
    );
    ctx.enable_vertex_attrib_array(a_texcoord as u32);

    let texture = ctx.create_texture();
    ctx.active_texture(WebGlRenderingContext::TEXTURE0);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );

    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    ctx.tex_image_2d_with_u32_and_u32_and_image_data(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGBA as i32,
        WebGlRenderingContext::RGBA,
        WebGlRenderingContext::UNSIGNED_BYTE,
        image_data,
    )?;

    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    ctx.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

    Ok(())
}
