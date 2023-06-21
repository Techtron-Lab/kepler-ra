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
use num::Num;
use nalgebra::{SMatrix, SVector};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Base {
    data: SMatrix<f32, 4, 4>,
    inv: Option<SMatrix<f32, 4, 4>>,
}

#[cfg(not(target_family = "wasm"))]
impl Base {
    pub fn from_row_slice(data: &[f32; 16]) -> Base {
        Base {
            data: SMatrix::<f32, 4, 4>::from_row_slice(data),
            inv: None,
        }
    }

    pub fn to_local(&mut self, v: &[f32; 3]) -> [f32; 3] {
        if self.inv.is_none() {
            let qr_decomp = self.data.qr();
            self.inv = qr_decomp.try_inverse();
        }

        assert!(self.inv.is_some());
        if let Some(m) = self.inv {
            let p0 = SVector::<f32, 4>::from_row_slice(&[v[0], v[1], v[2], 1.]);
            let p1 = m * (&p0);
            [p1[0]/p1[3], p1[1]/p1[3], p1[2]/p1[3]]
        } else {
            unreachable!()
        }
    }

    pub fn to_global(&self, v: &[f32; 3]) -> [f32; 3] {
        let p0 = SVector::<f32, 4>::from_row_slice(&[v[0], v[1], v[2], 1.]);
        let p1 = self.data * (&p0);
        [p1[0]/p1[3], p1[1]/p1[3], p1[2]/p1[3]]
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl Base {
    pub fn from_row_slice(data: &[f32]) -> Base {
        assert_eq!(data.len(), 16);
        Base {
            data: SMatrix::<f32, 4, 4>::from_row_slice(data),
            inv: None,
        }
    }

    pub fn to_local(&mut self, v: &[f32]) -> Box<[f32]> {
        assert_eq!(v.len(), 3);

        if self.inv.is_none() {
            let qr_decomp = self.data.qr();
            self.inv = qr_decomp.try_inverse();
        }
        assert!(self.inv.is_some());
        if let Some(m) = self.inv {
            let p0 = SVector::<f32, 4>::from_row_slice(&[v[0], v[1], v[2], 1.]);
            let p1 = m * (&p0);
            Box::new([p1[0]/p1[3], p1[1]/p1[3], p1[2]/p1[3]])
        } else {
            unreachable!()
        }
    }

    pub fn to_global(&self, v: &[f32]) -> Box<[f32]> {
        assert_eq!(v.len(), 3);
        let p0 = SVector::<f32, 4>::from_row_slice(&[v[0], v[1], v[2], 1.]);
        let p1 = self.data * (&p0);
        Box::new([p1[0]/p1[3], p1[1]/p1[3], p1[2]/p1[3]])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_base_creation() {
        let b = Base::from_row_slice(&[1.,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.,16.]);
        println!("{:?}", b);
    }
}
