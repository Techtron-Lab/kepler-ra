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


use std::ops::Deref;

use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::techtron::webgl::context::GLContext;

type GL2 = WebGl2RenderingContext;
pub trait LoadBuffer {
    type Target;
    fn load_buffer(&self, context: &GLContext) -> Result<Self::Target, String>;
}

pub struct VertexBuffer {
    pub buffer: Vec<f32>,
}

impl VertexBuffer {
    pub fn new(buffer: Vec<f32>) -> VertexBuffer {
        VertexBuffer { buffer }
    }
}

impl LoadBuffer for VertexBuffer {
    type Target = LoadedVertexBuffer;
    fn load_buffer(&self, context: &GLContext) -> Result<Self::Target, String> {
        let handle = context
            .create_buffer()
            .ok_or("failed to create vertices buffer")?;
        context.bind_buffer(GL2::ARRAY_BUFFER, Some(&handle));
        let vert_array = unsafe { js_sys::Float32Array::view(&self.buffer.as_slice()) };

        // context.vertex_attrib_pointer_with_i32(0, 3, GL2::FLOAT, false, 0, 0);
        // context.enable_vertex_attrib_array(0);

        context.buffer_data_with_array_buffer_view(
            GL2::ARRAY_BUFFER,
            &vert_array,
            GL2::STATIC_DRAW,
        );

        Ok(LoadedVertexBuffer {
            context: context.clone(),
            handle,
        })
    }
}

pub struct LoadedVertexBuffer {
    pub context: GLContext,
    pub handle: WebGlBuffer,
}

impl LoadedVertexBuffer {
    pub fn enable_buffer(&self) {
        self.context
            .bind_buffer(GL2::ELEMENT_ARRAY_BUFFER, Some(&self.handle));
    }
}

impl Drop for LoadedVertexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.handle));
    }
}

impl Deref for LoadedVertexBuffer {
    type Target = WebGlBuffer;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

pub struct IndexBuffer {
    pub buffer: Vec<u16>,
}

impl IndexBuffer {
    pub fn new(buffer: Vec<u16>) -> IndexBuffer {
        IndexBuffer { buffer }
    }
}

impl LoadBuffer for IndexBuffer {
    type Target = LoadedIndexBuffer;
    fn load_buffer(&self, context: &GLContext) -> Result<Self::Target, String> {
        let handle = context
            .create_buffer()
            .ok_or("failed to create vertices buffer")?;
        context.bind_buffer(GL2::ELEMENT_ARRAY_BUFFER, Some(&handle));
        let indices_array = unsafe { js_sys::Uint16Array::view(&self.buffer.as_slice()) };

        context.buffer_data_with_array_buffer_view(
            GL2::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL2::STATIC_DRAW,
        );      

        Ok(LoadedIndexBuffer {
            context: context.clone(),
            handle,
        })
    }
}

pub struct LoadedIndexBuffer {
    pub context: GLContext,
    pub handle: WebGlBuffer,
}

impl Drop for LoadedIndexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.handle));
    }
}

impl Deref for LoadedIndexBuffer {
    type Target = WebGlBuffer;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

pub struct Geometry {
    pub context: GLContext,
    pub vbuf: LoadedVertexBuffer,
    pub ibuf: LoadedIndexBuffer,
}

impl Geometry {
    pub fn enable_buffer(&self) {
        self.context.bind_buffer(GL2::ARRAY_BUFFER, Some(&self.vbuf.handle));
        self.context.bind_buffer(GL2::ELEMENT_ARRAY_BUFFER, Some(&self.ibuf.handle));
        self.context.vertex_attrib_pointer_with_i32(0, 3, GL2::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(0);
    }
}
