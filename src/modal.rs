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


use std::convert::TryInto;
use std::rc::Rc;

use js_sys::{ArrayBuffer, Int16Array, Uint16Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{File, FileReader};
use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader,
    WebGlTexture, //WebGlUniformLocation,
};

// struct volume_t {
//     dim: (usize, usize, usize),
//     channel: usize,
// }

// enum view_type_e {
//     transverse,
//     sagittal,
//     coronal,
//     bev,
//     three_d,
// }

// enum layout_e {
//     L3(view_type_e, view_type_e, view_type_e),
//     L2x2(view_type_e, view_type_e, view_type_e, view_type_e),
// }

// struct volume_view_t {
//     volume: volume_t,
//     dim: (usize, usize),
//     layout: layout_e,
// }

// struct Image3D {
//     data: Vec<u8>,
//     channel: i32,
//     dim: (i32, i32, i32),
// }

// impl Image3D {
//     pub fn new(width: usize, height: usize, depth: usize, channel: usize) -> Image3D {
//         let len = width * height * depth * channel;
//         let vec = Vec::<u8>::with_capacity(len);

//     }
// }

#[derive(Copy, Clone)]      
pub enum VolumeDataType {
    Undefined,
    Uint16,
    Int16,
}

#[derive(Clone)]
pub struct VolumeInfo {
    dimension: (i32, i32, i32),
    spacing: Option<(f32, f32, f32)>, // in millimeter
    data_type: VolumeDataType,
    slice_locations: Option<Vec<f32>>,
}

impl VolumeInfo {
    pub fn new(data_type: VolumeDataType, width: i32, height: i32, depth: i32) -> Self {
        VolumeInfo {
            dimension: (width, height, depth),
            spacing: None,
            data_type: data_type,
            slice_locations: None,
        }
    }

    pub fn get_data_type(&self) -> VolumeDataType {
        self.data_type
    }
    
    pub fn get_dimension(&self) -> (i32, i32, i32) {
        self.dimension
    }

    pub fn get_width(&self) -> i32 {
        self.dimension.0
    }

    pub fn get_height(&self) -> i32 {
        self.dimension.1
    }

    pub fn get_depth(&self) -> i32 {
        self.dimension.2
    }

    pub fn set_width(&mut self, width: i32) {
        self.dimension.0 = width;
    }

    pub fn set_height(&mut self, height: i32) {
        self.dimension.1 = height;
    }

    pub fn set_depth(&mut self, depth: i32) {
        self.dimension.2 = depth;
    }

    pub fn get_spacing(&self) -> Option<(f32, f32, f32)> {
        self.spacing
    }

    pub fn set_spacing(&mut self, spacing: (f32, f32, f32)) {
        self.spacing = Some(spacing);
    }

    pub fn get_spacing_x(&self) -> Option<f32> {
        if let Some((x, _y, _z)) = self.spacing {
            Some(x)
        } else {
            None
        }
    }

    pub fn get_spacing_y(&self) -> Option<f32> {
        if let Some((_x, y, _z)) = self.spacing {
            Some(y)
        } else {
            None
        }
    }

    pub fn get_spacing_z(&self) -> Option<f32> {
        if let Some((_x, _y, z)) = self.spacing {
            Some(z)
        } else {
            None
        }
    }

    pub fn get_slice_locations(&self) -> Option<&Vec<f32>> {
        self.slice_locations.as_ref()
    }
}

struct Volume<T> {
    data: Vec<T>,
}
