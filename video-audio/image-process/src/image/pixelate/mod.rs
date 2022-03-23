use crate::utils::{init_frame_buffer, GlContext};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{ImageData, WebGlBuffer, WebGlRenderbuffer, WebGlRenderingContext};

use super::vertices_attribute;

#[wasm_bindgen(js_name = pixelate)]
pub fn pixelate(image_data: &ImageData, ctx: &WebGlRenderingContext) -> Result<(), JsValue> {
    let mut gl_context = GlContext::new(ctx);
    gl_context.compile_shader(
        include_str!("./pixelate.vertex.glsl"),
        WebGlRenderingContext::VERTEX_SHADER,
    )?;
    gl_context.compile_shader(
        include_str!("./pixelate.fragment.glsl"),
        WebGlRenderingContext::FRAGMENT_SHADER,
    )?;
    gl_context.link_program()?;
    ctx.use_program(gl_context.program.as_ref());
    let scale = 16;
    let width = image_data.width() / scale;
    let height = image_data.height() / scale;
    let (frame_buffer, frame_texture) = init_frame_buffer(ctx, width as usize, height as usize)?;

    let buffer = ctx.create_buffer();
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, buffer.as_ref());
    unsafe {
        let vertex_attribute = js_sys::Float32Array::view(&vertices_attribute.clone());
        ctx.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_attribute,
            WebGlRenderingContext::STATIC_DRAW,
        )
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
    ctx.pixel_storei(WebGlRenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
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

    ctx.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&frame_buffer));
    ctx.viewport(0, 0, width as i32, height as i32);
    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    ctx.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
    ctx.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    ctx.viewport(0, 0, image_data.width() as i32, image_data.height() as i32);
    ctx.active_texture(WebGlRenderingContext::TEXTURE0);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&frame_texture));
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
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

    ctx.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
    Ok(())
}
