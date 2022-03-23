use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{ImageData, WebGlRenderingContext};

use crate::{
    image::vertices_attribute,
    utils::{set_panic_hook, GlContext},
};

#[wasm_bindgen(js_name=twist)]
pub fn twist(
    image_data: &ImageData,
    ctx: &WebGlRenderingContext,
    twist_val: f32,
) -> Result<(), JsValue> {
    set_panic_hook();
    let mut gl_context = GlContext::new(ctx);
    gl_context.compile_shader(
        include_str!("./twist.vertex.glsl"),
        WebGlRenderingContext::VERTEX_SHADER,
    )?;
    gl_context.compile_shader(
        include_str!("./twist.fragment.glsl"),
        WebGlRenderingContext::FRAGMENT_SHADER,
    )?;
    gl_context.link_program()?;
    ctx.use_program(gl_context.program.as_ref());

    let buffer = ctx.create_buffer().ok_or("create buffer failed")?;
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vertex_attribte = js_sys::Float32Array::view(&vertices_attribute.clone());
        ctx.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_attribte,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let file_size = 4;
    let a_position = ctx.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_Position");

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

    let u_iter = ctx.get_uniform_location(gl_context.program.as_ref().unwrap(), "u_Iter");
    ctx.uniform1f(u_iter.as_ref(), twist_val);

    let texture = ctx.create_texture();
    ctx.pixel_storei(WebGlRenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
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

    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    Ok(())
}
