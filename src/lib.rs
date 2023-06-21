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


#![allow(unused)]
mod colormap;
mod modal;
mod utils;
// mod textureid;
mod debug;
mod techtron;
mod view;
mod shader_sources;
mod glcanvas;

// Refactoring
// mod techtron;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::ops::DerefMut;
use std::rc::Rc;
// use js_sys::Math::{abs};
// use std::rc::Rc;

use do_notation::m;
use js_sys::{ArrayBuffer, Float32Array, Int16Array, Uint16Array, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::{File, FileReader};
use web_sys::{
    Performance,
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader,
    WebGlTexture, //WebGlUniformLocation,
};

// use crate::colormap::cmocean::LUT;
// use crate::colormap::jet::LUT;
// use crate::modal::{VolumeDataType, VolumeInfo};
// use techtron::prelude::*;
use crate::shader_sources::*;

use log::{info, warn, Level};
// pub use glcanvas::*;
pub use crate::techtron::prelude;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
