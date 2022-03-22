use std::{collections::HashMap, f32::consts::PI};

use new_string_template::template::Template;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use web_sys::{ImageData, WebGlRenderingContext};

use crate::{
    image::vertices_attribute,
    utils::{init_frame_buffer, GlContext},
};

fn gaussian(x: i32, sigma: f32) -> f32 {
    let temp = 1. / ((PI * 2.).sqrt() * sigma);
    temp * (-((x as f32).powi(2)) / (2. * sigma.powi(2))).exp()
}

#[wasm_bindgen(js_name=gaussianBlur)]
pub fn gaussian_blur(
    image_data: &ImageData,
    ctx: &WebGlRenderingContext,
    window_size: usize,
) -> Result<(), JsValue> {
    let sigma = 0.3 * ((window_size as f32) * 0.5 - 1.) + 0.8;
    let mut kernel: Vec<f32> = vec![];
    let centre = window_size / 2;
    for i in 0..window_size {
        kernel.push(gaussian(i as i32 - centre as i32, sigma))
    }
    // web_sys::console::log_1(&format!("{:?}", kernel).into());
    let mut gl_context = GlContext::new(ctx);

    gl_context.compile_shader(
        include_str!("./gaussian_blur.vertex.glsl"),
        WebGlRenderingContext::VERTEX_SHADER,
    )?;

    let fragment_shader_template = include_str!("./gaussian_blur.fragment.glsl");
    let templ = Template::new(fragment_shader_template);

    let fragment_shader_source = {
        let mut map = HashMap::new();
        map.insert("kernel_size", window_size.to_string());
        let rendered = templ.render(&map);
        if !rendered.is_ok() {
            return Err(JsValue::from_str(&format!("template error {:?}", rendered)));
        }
        rendered.unwrap()
    };
    // web_sys::console::log_1(&format!("{:?}", fragment_shader_source).into());
    gl_context.compile_shader(
        &fragment_shader_source,
        WebGlRenderingContext::FRAGMENT_SHADER,
    )?;

    gl_context.link_program()?;
    ctx.use_program(gl_context.program.as_ref());
    let (frame_buffer, frame_buffer_texture) = init_frame_buffer(
        ctx,
        image_data.width() as usize,
        image_data.height() as usize,
    )?;

    let buffer = ctx.create_buffer().ok_or("create buffer err".to_string())?;
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    let file_size = 4;
    unsafe {
        let vertx_attrib_buffer = js_sys::Float32Array::view(&vertices_attribute.clone());
        ctx.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertx_attrib_buffer,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    let a_position = ctx.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_Position");
    ctx.vertex_attrib_pointer_with_i32(
        a_position as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        (file_size * 4) as i32,
        0,
    );
    ctx.enable_vertex_attrib_array(a_position as u32);

    let a_tex_coord = ctx.get_attrib_location(gl_context.program.as_ref().unwrap(), "a_TexCoord");

    ctx.vertex_attrib_pointer_with_i32(
        a_tex_coord as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        (file_size * 4) as i32,
        file_size * 2,
    );
    ctx.enable_vertex_attrib_array(a_tex_coord as u32);
    // ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    let u_texture_size = ctx
        .get_uniform_location(gl_context.program.as_ref().unwrap(), "u_TextureSize")
        .unwrap();
    let u_horizontal = ctx
        .get_uniform_location(gl_context.program.as_ref().unwrap(), "u_Horizontal")
        .unwrap();
    ctx.uniform2f(
        Some(&u_texture_size),
        image_data.width() as f32,
        image_data.height() as f32,
    );
    ctx.uniform1i(Some(&u_horizontal), 1);
    let u_kernel = ctx
        .get_uniform_location(gl_context.program.as_ref().unwrap(), "u_Kernel")
        .unwrap();
    ctx.uniform1fv_with_f32_array(Some(&u_kernel), &kernel);
    let texture = ctx
        .create_texture()
        .ok_or("create texture err".to_string())?;
    let u_sampler = ctx
        .get_uniform_location(gl_context.program.as_ref().unwrap(), "u_Sampler")
        .ok_or("get u_Sampler uniform err".to_string())?;
    ctx.pixel_storei(WebGlRenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
    ctx.active_texture(WebGlRenderingContext::TEXTURE0);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    ctx.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
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
    ctx.uniform1i(Some(&u_sampler), 0);

    ctx.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&frame_buffer));
    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    ctx.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
    ctx.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
    ctx.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);

    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    ctx.uniform1i(Some(&u_horizontal), 0);
    ctx.active_texture(WebGlRenderingContext::TEXTURE0);
    ctx.bind_texture(
        WebGlRenderingContext::TEXTURE_2D,
        Some(&frame_buffer_texture),
    );

    ctx.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use new_string_template::template::Template;
    #[test]
    fn test_template() {
        let templ_str = include_str!("./gaussian_blur.fragment.glsl");
        let data = {
            let mut map = HashMap::new();
            map.insert("kernel", "1,2,3,4,5".to_string());
            map.insert("kernel_size", 5.to_string());
            map
        };
        let templ = Template::new(templ_str);
        let rendered = templ.render(&data).unwrap();
        println!("renderd ---> {:?}", rendered);
    }
}
