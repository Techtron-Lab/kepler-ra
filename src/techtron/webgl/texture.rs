
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


use log::info;
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use super::context::*;

type GL2 = WebGl2RenderingContext;

// pub fn gen_texture_id() -> Result<u32, String> {
//     // static TEXTURE_ID: AtomicU32 = AtomicU32::new(WebGl2RenderingContext::TEXTURE10);
//     static TEXTURE_ID: AtomicU32 = AtomicU32::new(10);
//     let id = TEXTURE_ID.fetch_add(1, Ordering::SeqCst);
//     if id < WebGl2RenderingContext::MAX_COMBINED_TEXTURE_IMAGE_UNITS {
//         Ok(id)
//     } else {
//         Err(String::from("Texture id is too large."))
//     }
// }

const MAX_TEXTURE_ID: u32 =
    WebGl2RenderingContext::MAX_COMBINED_TEXTURE_IMAGE_UNITS - WebGl2RenderingContext::TEXTURE0 - 1;

static TEXTURE_ID: Lazy<Mutex<Vec<u32>>> =
    Lazy::new(|| Mutex::new((0..MAX_TEXTURE_ID).rev().collect()));

pub fn gen_texture_id() -> Result<u32, String> {
    TEXTURE_ID
        .lock()
        .or(Err(String::from("Cannot lock TEXTURE_ID.")))
        .and_then(|mut v| v.pop().ok_or(String::from("Texture id exhausted.")))
}

pub fn release_texture_id(id: u32) -> Result<(), String> {
    if id < MAX_TEXTURE_ID {
        TEXTURE_ID
        .lock()
        .or(Err(String::from("Cannot lock TEXTURE_ID.")))
        .and_then(|mut v| Ok(v.push(id)))
    } else {
        Err(String::from("Texture id is out of range."))
    }
}

pub trait LoadTexture {
    fn load_texture(&self, context: &GLContext) -> LoadedTexture;
}

pub struct Texture3DRGBA16 {
    width: i32,
    height: i32,
    depth: i32,
    data: Rc<Vec<u16>>,
}

impl Texture3DRGBA16 {
    pub fn new(width: i32, height: i32, depth: i32, data: Rc<Vec<u16>>) -> Self {
        Texture3DRGBA16 {
            width,
            height,
            depth,
            data,
        }
    }
    
}

impl LoadTexture for Texture3DRGBA16 {
    fn load_texture(&self, context: &GLContext) -> LoadedTexture {
        let gl = context.clone();
        let level = 0;
        let border = 0;
        let internal_format = GL2::RGBA as i32;
        let source_format = GL2::RGBA;
        let source_type = GL2::UNSIGNED_SHORT_4_4_4_4;
        let id = gen_texture_id().expect("Cannot generate texture id.");

        // create texture
        let handle = gl.create_texture().expect("Failed to crate texture.");
        gl.bind_texture(GL2::TEXTURE_3D, Some(&handle));

        // copy data go GPU
        let data: &[u16] = self.data.as_slice();
        let array = unsafe { &js_sys::Uint16Array::view(data) };

        gl.tex_image_3d_with_opt_array_buffer_view(
            GL2::TEXTURE_3D,
            level,
            internal_format,
            self.width,
            self.height,
            self.depth,
            border,
            source_format,
            source_type,
            Some(array),
        );
        let error = gl.get_error();
        if error != 0 {
            panic!("copying error: {}.", error);
        } else {
            info!("finish copying.");
        }

        set_default_texture_param(&gl);
        LoadedTexture {
            context: context.clone(),
            handle,
            id,
        }
        
    }
}

#[derive(Debug)]
pub struct LoadedTexture {
    pub context: GLContext,
    pub handle: WebGlTexture,
    pub id: u32,
    // pub data: Rc<CTVolume>,
    // pub data: Vec<u8>,
}
impl Drop for LoadedTexture {
    fn drop(&mut self) {
        // info!("delete loaded texture: {}", &self.id);
        release_texture_id(self.id);
        self.context.delete_texture(Some(&self.handle));
    }
}

impl Deref for LoadedTexture {
    type Target = WebGlTexture;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl LoadedTexture {
    pub fn activate(&self) {
        self.context.active_texture(GL2::TEXTURE0 + self.id)
    }
}

pub fn set_default_texture_param(gl: &WebGl2RenderingContext) {
    type GL2 = WebGl2RenderingContext;
    // gl.bind_texture(GL2::TEXTURE_3D, texture);
    gl.tex_parameteri(
        GL2::TEXTURE_3D,
        GL2::TEXTURE_WRAP_S,
        GL2::CLAMP_TO_EDGE.try_into().unwrap(),
    );
    gl.tex_parameteri(
        GL2::TEXTURE_3D,
        GL2::TEXTURE_WRAP_T,
        GL2::CLAMP_TO_EDGE.try_into().unwrap(),
    );
    gl.tex_parameteri(
        GL2::TEXTURE_3D,
        GL2::TEXTURE_WRAP_R,
        GL2::CLAMP_TO_EDGE.try_into().unwrap(),
    );
    gl.tex_parameteri(
        GL2::TEXTURE_3D,
        GL2::TEXTURE_MIN_FILTER,
        GL2::LINEAR.try_into().unwrap(),
        // GL2::NEAREST.try_into().unwrap(),
    );
    gl.tex_parameteri(
        GL2::TEXTURE_3D,
        GL2::TEXTURE_MAG_FILTER,
        GL2::LINEAR.try_into().unwrap(),
        // GL2::NEAREST.try_into().unwrap(),
    );
}


pub trait GenTexture<T> {
    fn gen_texture3d(&self) -> T;
}

pub struct Texture3DRGB8 {
    width: i32,
    height: i32,
    depth: i32,
    data: Rc<Vec<u8>>,
}

impl Texture3DRGB8 {
    pub fn new(width: i32, height: i32, depth: i32, data: Rc<Vec<u8>>) -> Self {
        Texture3DRGB8 {
            width,
            height,
            depth,
            data,
        }
    }
    
}

impl LoadTexture for Texture3DRGB8 {
    fn load_texture(&self, context: &GLContext) -> LoadedTexture {
        let gl = context.clone();
        // let (w, h, d) = (256, 1, 1);
        let border = 0;
        let level = 0;

        let data = self.data.as_slice();
        let lut_array_view = unsafe { js_sys::Uint8Array::view(&data) };
        let handle = gl.create_texture().expect("Failed to create LUT texture.");
        let id = gen_texture_id().expect("Cannot generate texture id.");
        gl.bind_texture(GL2::TEXTURE_3D, Some(&handle));
        gl.tex_image_3d_with_opt_array_buffer_view(
            GL2::TEXTURE_3D,
            level,
            GL2::RGB8 as i32,
            self.width,
            self.height,
            self.depth,
            border,
            GL2::RGB,
            GL2::UNSIGNED_BYTE,
            Some(&lut_array_view),
        )
        .expect("Failed to copy LUT data to GPU.");
        set_default_texture_param(&gl);

        LoadedTexture {
            context: context.clone(),
            handle,
            id,
        }
    }
}
