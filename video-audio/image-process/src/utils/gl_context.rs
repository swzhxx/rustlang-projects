use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

pub struct GlContext<'a> {
    pub ctx: &'a WebGlRenderingContext,
    pub program: Option<WebGlProgram>,
    vert_shader: Option<WebGlShader>,
    frag_shader: Option<WebGlShader>,
}

impl<'a> GlContext<'a> {
    pub fn new(ctx: &'a WebGlRenderingContext) -> Self {
        Self {
            ctx,
            program: None,
            vert_shader: None,
            frag_shader: None,
        }
    }
    /// 编译shader
    pub fn compile_shader(&mut self, source: &str, shader_type: u32) -> Result<(), String> {
        let shader = self
            .ctx
            .create_shader(shader_type)
            .ok_or_else(|| "Unable to create Shader".to_string())?;
        self.ctx.shader_source(&shader, source);
        self.ctx.compile_shader(&shader);

        if self
            .ctx
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            match shader_type {
                WebGlRenderingContext::FRAGMENT_SHADER => self.frag_shader = Some(shader),
                WebGlRenderingContext::VERTEX_SHADER => self.vert_shader = Some(shader),
                _ => {}
            }
            Ok(())
        } else {
            Err(self
                .ctx
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".to_string()))
        }
    }

    pub fn link_program(&mut self) -> Result<(), String> {
        let program = self
            .ctx
            .create_program()
            .ok_or_else(|| "Unable to create shader Object".to_string())?;
        if self.vert_shader.is_none() || self.frag_shader.is_none() {
            return Err("Unable to link program , your shader uncomplied".to_string());
        }
        self.ctx
            .attach_shader(&program, self.vert_shader.as_ref().unwrap());
        self.ctx
            .attach_shader(&program, self.frag_shader.as_ref().unwrap());
        self.ctx.link_program(&program);
        if !self
            .ctx
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let info = self.ctx.get_program_info_log(&program);
            return Err(format!("link program err {:?}", info));
        }
        self.program = Some(program);
        Ok(())
    }
}
