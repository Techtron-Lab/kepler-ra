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


// #[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
use js_sys::{Array, ArrayBuffer, Float64Array};

#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;

use super::Point;

use log::info;

//     [3]    3     [2]
//      +------------+
//      |            |
//    0 |            | 2
//      |            |
//      +------------+
//     [0]    1      [1]
#[rustfmt::skip]
const MARCHING_SQUARES_LUT: [[i8; 4]; 16] = [
    [-1, -1, -1, -1],
    [ 0,  1, -1, -1],
    [ 1,  2, -1, -1],
    [ 0,  2, -1, -1],
    [ 2,  3, -1, -1],
    [ 0,  3,  1,  2],
    [ 1,  3, -1, -1],
    [ 0,  3, -1, -1],
    [ 0,  3, -1, -1],
    [ 1,  3, -1, -1],
    [ 0,  1,  2,  3],
    [ 2,  3, -1, -1],
    [ 0,  2, -1, -1],
    [ 1,  2, -1, -1],
    [ 0,  1, -1, -1],
    [-1, -1, -1, -1],
];

type Point2D<T> = [T; 2];

fn marching_squares_impl<T>(isovalue: T, data: &[T], width: i32, height: i32) -> Vec<[Point2D<T>; 2]>
where
    T: std::cmp::PartialOrd
        // + std::convert::From<i16>
        + std::ops::Add<T, Output = T>
        + std::convert::From<f32>,
{
    info!("width: {}, height: {}", width, height);
    let mut line_segments: Vec<[Point2D<T>; 2]> = Vec::new();
    for y in 0..height - 2 {
        for x in 0..width - 2 {
            let idx0 = ((y + 1) * width + x) as usize;
            let idx1 = ((y + 1) * width + x + 1) as usize;
            let idx2 = (y * width + x + 1) as usize;
            let idx3 = (y * width + x) as usize;
            // info!("idx0: {}, idx1: {}, idx2: {}, idx3: {}", idx0, idx1, idx2, idx3);
            let v0 = if data[idx0] > isovalue { 1 } else { 0 };
            let v1 = if data[idx1] > isovalue { 1 } else { 0 };
            let v2 = if data[idx2] > isovalue { 1 } else { 0 };
            let v3 = if data[idx3] > isovalue { 1 } else { 0 };

            if (v0 == 0 && v1 == 0 && v2 == 0 && v3 == 0)
                || (v0 == 1 && v1 == 1 && v2 == 1 && v3 == 1)
            {
                continue;
            }

            let idx = v0 | v1 << 1 | v2 << 2 | v3 << 3;
            // info!("{} {} {} {} index: {}", v0, v1, v2, v3, idx);
            let lines = MARCHING_SQUARES_LUT[idx];
            if lines[0] == -1 {
                continue;
            }
            for i in (0..4).step_by(2) {
                let a = lines[i];
                if a == -1 { break; }
                let b = lines[i + 1];
                info!("a: {}, b: {}", a, b);
                let p0 = adjust_coord::<T>(a, Into::<T>::into(x as f32), Into::<T>::into(y as f32));
                let p1 = adjust_coord::<T>(b, Into::<T>::into(x as f32), Into::<T>::into(y as f32));
                line_segments.push([p0, p1]);
            }
        }
    }
    line_segments
}

fn adjust_coord<T>(n: i8, x: T, y: T) -> [T; 2]
where
    T: std::ops::Add<T, Output = T> + std::convert::From<f32>,
{
    // let x0 = x.into();
    // let y0 = y.into();
    match n {
        // 0 => [x0, y0 + 0.5],
        // 1 => [x0 + 0.5, y0 + 1.],
        // 2 => [x0 + 1., y0 + 0.5],
        // 3 => [x0 + 0.5, y0],

        0 => [x, y + Into::<T>::into(0.5)],
        1 => [x + Into::<T>::into(0.5), y + Into::<T>::into(1.)],
        2 => [x + Into::<T>::into(1.), y+ Into::<T>::into(0.5)],
        3 => [x + Into::<T>::into(0.5), y],
        _ => unreachable!(),
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn marching_squares(
    isovalue: f32,
    slice: &Slice
    // data: Vec<f32>,
    // width: i16,
    // height: i16,
) -> Result<Array, String> {
    let width = slice.width();
    let height = slice.height();
    let data = slice.data();
    let v = marching_squares_impl::<f32>(isovalue, &data, width, height);
    let mut arr = Array::new();

    // assemble the line segments in a JS Array
    for line in v {
        let p0 = line[0];
        let p1 = line[1];

        let mut p0_js = Array::new_with_length(2);
        p0_js.set(0, JsValue::from(p0[0]));
        p0_js.set(1, JsValue::from(p0[1]));

        let mut p1_js = Array::new_with_length(2);
        p1_js.set(0, JsValue::from(p1[0]));
        p1_js.set(1, JsValue::from(p1[1]));

        let mut line_js = Array::new_with_length(2);
        line_js.set(0, JsValue::from(p0_js));
        line_js.set(1, JsValue::from(p1_js));

        arr.push(&line_js);
    }
    Ok(arr)
}

#[cfg(not(target_family = "wasm"))]
pub fn marching_squares(
    isovalue: f64,
    data: &[f64],
    width: i32,
    height: i32,
) -> Vec<[Point2D<f64>; 2]> {
    marching_squares_impl::<f64>(isovalue, data, width, height)
}

// #[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub struct Slice {
    dim: (i32, i32),
    spacing: (f32, f32),
    data: Vec<f32>,
}

#[wasm_bindgen]
impl Slice {
    pub fn new(dim_x: i32, dim_y: i32, spacing_x: f32, spacing_y: f32, data: Vec<f32>) -> Slice {
        Slice {
            dim: (dim_x, dim_y),
            spacing: (spacing_x, spacing_y),
            data
        }
    }

    pub fn width(&self) -> i32 {
        self.dim.0
    }

    pub fn height(&self) -> i32 {
        self.dim.1
    }

    fn data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn dim(&self) -> Box<[i32]> {
        let mut v: Box<[i32]> = Box::new([0, 0]);
        v[0] = self.dim.0;
        v[1] = self.dim.1;
        v
    }

    pub fn spacing(&self) -> Box<[f32]> {
        let mut v: Box<[f32]> = Box::new([0., 0.]);
        v[0] = self.spacing.0;
        v[1] = self.spacing.1;
        v
    }
}

// fn sample_grid() {

// }
