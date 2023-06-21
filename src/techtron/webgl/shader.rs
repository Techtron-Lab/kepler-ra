// MIT License

// Copyright (c) 2023 Techtron-Lab

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use js_sys::{Array, ArrayBuffer, Float32Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    // Performance,
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader,
    WebGlTexture,
};

use super::texture::LoadedTexture;

use super::{context::GLContext, program::Program};

type GL2 = WebGl2RenderingContext;

#[derive(Debug, Clone)]
pub struct ShaderSource(pub String);

impl ShaderSource {
    pub fn new(source: &str) -> Self {
        Self(String::from(source))
    }

    pub fn to_vertex(&self) -> VertexShader {
        VertexShader {
            source: self.clone(),
        }
    }

    pub fn to_fragment(&self) -> FragmentShader {
        FragmentShader {
            source: self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexShader {
    pub source: ShaderSource,
}

impl VertexShader {
    pub fn compile(&self, context: &GLContext) -> Result<CompiledVertexShader, String> {
        let handle = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            &self.source.0,
        )?;
        Ok(CompiledVertexShader {
            handle,
            context: context.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FragmentShader {
    pub source: ShaderSource,
}

impl FragmentShader {
    pub fn compile(&self, context: &GLContext) -> Result<CompiledFragmentShader, String> {
        let handle = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &self.source.0,
        )?;
        Ok(CompiledFragmentShader {
            handle,
            context: context.clone(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompiledVertexShader {
    pub handle: WebGlShader,
    pub context: GLContext,
}

impl Drop for CompiledVertexShader {
    fn drop(&mut self) {
        self.context.delete_shader(Some(&self.handle))
    }
}

impl Deref for CompiledVertexShader {
    type Target = WebGlShader;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledFragmentShader {
    pub handle: WebGlShader,
    pub context: GLContext,
}

impl Drop for CompiledFragmentShader {
    fn drop(&mut self) {
        self.context.delete_shader(Some(&self.handle))
    }
}

impl Deref for CompiledFragmentShader {
    type Target = WebGlShader;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let shader_type_str = match shader_type {
            WebGl2RenderingContext::FRAGMENT_SHADER => "Fragment shader: ",
            WebGl2RenderingContext::VERTEX_SHADER => "Vertex shader: ",
            _ => panic!("never be here!"),
        };
        Err(format!(
            "{}{}",
            shader_type_str,
            context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader"))
        ))
    }
}

pub fn load_shaders(
    context: &WebGl2RenderingContext,
    vert_src: &str,
    frag_src: &str,
) -> Result<Program, JsValue> {
    let context = GLContext::new(context.clone());
    let vert_shader = ShaderSource::new(vert_src).to_vertex().compile(&context)?;
    let frag_shader = ShaderSource::new(frag_src)
        .to_fragment()
        .compile(&context)?;
    let program = Program::new(&context, vert_shader, frag_shader);
    // context.0.use_program(Some(&program));
    Ok(program)
}
