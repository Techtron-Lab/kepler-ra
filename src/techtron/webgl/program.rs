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
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::techtron::webgl::texture::LoadedTexture;

use super::context::GLContext;
use super::shader::{CompiledFragmentShader, CompiledVertexShader};
use super::super::log::*;
use crate::console;

type GL2 = WebGl2RenderingContext;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub vertex: CompiledVertexShader,
    pub fragment: CompiledFragmentShader,
    pub handle: WebGlProgram,
    pub context: GLContext,
}

impl Program {
    pub fn new(
        context: &GLContext,
        vertex: CompiledVertexShader,
        fragment: CompiledFragmentShader,
    ) -> Program {
        if vertex.context != fragment.context || *context != vertex.context {
            todo!()
        }
        let context = context.clone();
        let handle = link_program(&context, &*vertex, &*fragment).unwrap();
        Program {
            vertex,
            fragment,
            context,
            handle,
        }
    }

    pub fn use_program(&self) {
        self.context.use_program(Some(&self.handle));
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.context.detach_shader(&self, &self.vertex);
        self.context.detach_shader(&self, &self.fragment);
        self.context.delete_program(Some(&self.handle))
    }
}

impl Deref for Program {
    type Target = WebGlProgram;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

// impl DerefMut for Program {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.handle
//     }
// }

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

impl Program {
    pub fn set_uniform1i(&self, uniform_name: &str, value: i32) -> Result<(), String> {
        let gl = &self.context;
        let u_var = gl.get_uniform_location(&self, uniform_name);
        gl.uniform1i(u_var.as_ref(), value);
        Ok(())
    }

    pub fn get_uniform1i(&self, uniform_name: &str) -> Result<i32, String> {
        let gl = &self.context;
        let u_var = gl.get_uniform_location(&self, uniform_name).ok_or(format!(
            "Cannot retrieve uniform:{} location.",
            uniform_name
        ))?;
        let value: i32 = gl
            .get_uniform(&self, u_var.as_ref())
            .as_f64()
            .ok_or(format!(
                "Cannot retrieve uniform: {} as a float.",
                uniform_name
            ))? as i32;
        Ok(value)
    }

    pub fn set_uniform1f(&self, uniform_name: &str, value: f32) -> Result<(), String> {
        let gl = &self.context;
        let u_var = gl.get_uniform_location(&self, uniform_name);
        gl.uniform1f(u_var.as_ref(), value);
        Ok(())
    }

    pub fn get_uniform1f(&self, uniform_name: &str) -> Result<f32, String> {
        let gl = &self.context;
        let u_var = gl.get_uniform_location(&self, uniform_name).ok_or(format!(
            "Cannot retrieve uniform:{} location.",
            uniform_name
        ))?;
        let value: f32 = gl
            .get_uniform(&self, u_var.as_ref())
            .as_f64()
            .ok_or(format!(
                "Cannot retrieve uniform: {} as a float.",
                uniform_name
            ))? as f32;
        Ok(value)
    }

    pub fn set_uniform3f(&self, u_name: &str, v0: f32, v1: f32, v2: f32) -> Result<(), String> {
        let gl = &self.context;
        let u_var = gl.get_uniform_location(&self, u_name);
        gl.uniform3f(u_var.as_ref(), v0, v1, v2);
        Ok(())
    }

    pub fn get_uniform3f(&self, u_name: &str) -> Result<Box<[f32]>, String> {
        let gl = &self.context;
        let u_var = gl
            .get_uniform_location(&self, u_name)
            .ok_or(format!("Cannot retrieve uniform:{} location", u_name))?;
        let v: ArrayBuffer = gl
            .get_uniform(&self, u_var.as_ref())
            .dyn_into()
            .map_err(|err| format!("cannot retrieve uniform: {:?}", err))?;
        let ret = Float32Array::new(&v).to_vec().into_boxed_slice();
        Ok(ret)
    }

    pub fn bind_texture_uniform(
        &self,
        texture: &LoadedTexture,
        u_name: &str,
    ) -> Result<(), String> {
        // console!("entering bind_texture_uniform: {:?} {}", &texture, u_name);
        let gl = &self.context;
        let texture_unit = texture.id;

        texture.activate();
        gl.bind_texture(GL2::TEXTURE_3D, Some(&texture));

        let sampler = gl.get_uniform_location(self, u_name);
        match sampler {
            Some(_) => gl.uniform1i(sampler.as_ref(), texture_unit as i32),
            None => {
                let error = gl.get_error();
                let err_str = format!(
                    "cannot locate uniform: {}. Error number: {} with texture {}",
                    u_name, error, texture.id
                );
                console!("{}", err_str);
                return Err(err_str);
            }
        }

        // console!("bind texture {} with program uniform {}", texture.id, u_name);

        Ok(())
    }
}
