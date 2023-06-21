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


#![allow(non_snake_case)]

use nalgebra::*;
use wasm_bindgen::prelude::wasm_bindgen;

// Optimization

pub fn steepest_decent(
    A: &DMatrix<f64>,
    x: &DVector<f64>,
    b: &DVector<f64>,
    epsilon: f64,
    max_iter: usize,
) -> DVector<f64> {
    let mut x: DVector<f64> = x.clone();
    let mut i = 0;
    let mut r = b - A * &x;
    let mut delta = (r.tr_mul(&r))[0];
    // let delta0 = delta;

    // while i < max_iter && delta > 1e-12 * delta0 {
    while i < max_iter && delta > epsilon {
        let q = A * &r;
        let alpha = delta / (r.tr_mul(&q))[0];
        x += alpha * &r;
        if i % 50 == 0 {
            r = b - A * &x;
        } else {
            r -= alpha * &q;
        }
        delta = (r.tr_mul(&r))[0];
        i += 1;
    }
    // console_log!("delta: {:e}", delta);
    // console_log!("iter: {}", i);
    // console_log!("x: {}", x);
    // console::log_1(&JsValue::from(format!("r: {}", r)));

    return x;
}

pub fn conjugated_gradient(
    A: &DMatrix<f64>,
    x: &DVector<f64>,
    b: &DVector<f64>,
    epsilon: f64,
    max_iter: usize,
) -> DVector<f64> {
    let mut x: DVector<f64> = x.clone();
    let mut i = 0;
    let mut r = b - A * &x;
    let mut d = r.clone();
    let mut delta_new = (r.tr_mul(&r))[0];
    let mut delta_old = delta_new;
    // let delta0 = delta;

    // while i < max_iter && delta > 1e-12 * delta0 {
    while i < max_iter && delta_new > epsilon {
        let q = A * &d;
        let alpha = delta_new / (d.tr_mul(&q))[0];
        x += alpha * &d;
        if i % 50 == 0 {
            r = b - A * &x;
        } else {
            r -= alpha * &q;
        }
        delta_old = delta_new;
        delta_new = (r.tr_mul(&r))[0];
        let beta = delta_new / delta_old;
        d = &r + beta * &d;
        i += 1;
    }
    // console_log!("delta: {:e}", delta_new);
    // console_log!("iter: {}", i);
    // console_log!("x: {}", &x);
    // console::log_1(&JsValue::from(format!("r: {}", r)));

    return x;
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Base3D {
    matrix: Matrix4<f32>,
}

#[wasm_bindgen]
impl Base3D {
    pub fn new_with_identity() -> Base3D {
        Base3D {
            matrix: Matrix4::<f32>::identity(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
}


