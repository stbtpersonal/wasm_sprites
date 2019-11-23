use web_sys::{
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

use crate::canvas::Canvas;

pub struct Shader {
    program: WebGlProgram
}

impl Shader {
    pub fn new(canvas: &Canvas, vertex_shader_source: &str, fragment_shader_source: &str) -> Shader {
        let program = Shader::compile_program(canvas, vertex_shader_source, fragment_shader_source);

        Shader {
            program
        }
    }

    fn compile_program(canvas: &Canvas, vertex_shader_source: &str, fragment_shader_source: &str) -> WebGlProgram {
        let vertex_shader = Shader::compile_vertex_shader(canvas, vertex_shader_source);
        let fragment_shader = Shader::compile_fragment_shader(canvas, fragment_shader_source);

        let gl = canvas.gl();
        let program = gl.create_program().unwrap();
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        let program_link_status = gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap();
        if !program_link_status {
            let error = gl.get_program_info_log(&program).unwrap();
            panic!("Program link failed: {}", error);
        }

        program
    }

    fn compile_vertex_shader(canvas: &Canvas, source: &str) -> WebGlShader {
        Shader::compile_shader(canvas, WebGlRenderingContext::VERTEX_SHADER, source)
    }

    fn compile_fragment_shader(canvas: &Canvas, source: &str) -> WebGlShader {
        Shader::compile_shader(canvas, WebGlRenderingContext::FRAGMENT_SHADER, source)
    }

    fn compile_shader(canvas: &Canvas, shader_type: u32, source: &str) -> WebGlShader {
        let gl = canvas.gl();
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        let shader_compile_status = gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap();
        if !shader_compile_status {
            let error = gl.get_shader_info_log(&shader).unwrap();
            panic!("Shader compilation failed: {}", error)
        }

        shader
    }

    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }
}