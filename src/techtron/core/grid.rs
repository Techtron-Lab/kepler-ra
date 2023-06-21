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


use std::ops::Add;

use log::info;
use num::Num;


#[derive(Debug, Clone)]
pub struct Grid2D<T> {
    dim: [usize; 2],
    data: Vec<T>,
}

impl <T> Grid2D<T> 
where 
    T: Copy + Num 
{
    pub fn new(nx: usize, ny: usize) -> Grid2D<T> {
        let size: usize = nx * ny;
        let mut data = Vec::<T>::with_capacity(size);
        data.resize(size, T::zero());
        Grid2D {
            dim: [nx, ny],
            data,
        }
    }

    // Create Grid2D from a slice.
    pub fn from_raw_data(data: &[T], nx: usize, ny: usize) -> Option<Grid2D<T>> {
        if data.len() == nx * ny {
            Some(Grid2D {
                dim: [nx, ny],
                data: Vec::from(data),
            })
        } else {
            None
        }
    }
    
    pub fn dim(&self) -> [usize; 2] {
        self.dim
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    
    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        // info!("x = {}, dim[0] = {}, y = {}, dim[1] = {}", x, self.dim[0], y, self.dim[1]);
        assert!(x < self.dim[0]);
        assert!(y < self.dim[1]);
        
        self.dim[0] * y + x
    }

    #[inline]
    pub fn value_at(&self, x: usize, y: usize) -> T {
        self.data[self.idx(x, y)] 
    }

    #[inline]
    pub fn set_value_at(&mut self, x: usize, y: usize, v: &T) {
        let i = self.idx(x, y);
        self.data[i] = *v;
    }
}

impl <T> Add for Grid2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct Grid3D<T> {
    dim: [usize; 3],
    data: Vec<T>,
}

impl<T> Grid3D<T>
where
    T: Copy + Num,
{
    pub fn new(nx: usize, ny: usize, nz: usize) -> Grid3D<T> {
        let size: usize = (nx * ny * nz) as usize;
        let mut data = Vec::<T>::with_capacity(size);
        data.resize(size, T::zero());
        Grid3D {
            dim: [nx, ny, nz],
            data,
        }
    }

    // Create Grid3D from a slice.
    pub fn from_raw_data(data: &[T], nx: usize, ny: usize, nz: usize) -> Option<Grid3D<T>> {
        if data.len() == (nx * ny * nz) as usize {
            Some(Grid3D {
                dim: [nx, ny, nz],
                data: Vec::from(data),
            })
        } else {
            None
        }
    }
    
    pub fn dim(&self) -> [usize; 3] {
        self.dim
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    
    #[inline]
    fn idx(&self, x: usize, y: usize, z: usize) -> usize {
        assert!(x < self.dim[0]);
        assert!(y < self.dim[1]);
        assert!(z < self.dim[2]);
        
        self.dim[0] * self.dim[1] * z + self.dim[0] * y + x
    }

    #[inline]
    pub fn value_at(&self, x: usize, y: usize, z: usize) -> T {
        self.data[self.idx(x, y, z)] 
    }

    #[inline]
    pub fn set_value_at(&mut self, x: usize, y: usize, z: usize, v: &T) {
        let i = self.idx(x, y, z);
        self.data[i] = *v;
    }

    #[inline]
    pub fn slice_xy_at(z: usize) -> Grid2D<T> {
        todo!()
    }

    #[inline]
    pub fn slice_yz_at(z: usize) -> Grid2D<T> {
        todo!()
    }

    #[inline]
    pub fn slice_xz_at(z: usize) -> Grid2D<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grid2d_creation() {
        let nx = 512;
        let ny = 500;
        let g = Grid2D::<f64>::new(nx, ny);
        assert_eq!(g.dim[0], nx);
        assert_eq!(g.dim[1], ny);
        assert_eq!(g.data.len(), nx * ny);
    }
    
    #[test]
    fn test_grid2d_idx() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let g0 = Grid2D::<u32>::from_raw_data(&data, 2, 2);
        assert!(g0.is_none());
        let g = Grid2D::<u32>::from_raw_data(&data, 3, 3).unwrap();
        assert_eq!(g.value_at(0, 0), 1);
        assert_eq!(g.value_at(1, 0), 2);
        assert_eq!(g.value_at(2, 0), 3);
        assert_eq!(g.value_at(0, 1), 4);
        assert_eq!(g.value_at(1, 1), 5);
        assert_eq!(g.value_at(2, 1), 6);
        assert_eq!(g.value_at(0, 2), 7);
        assert_eq!(g.value_at(1, 2), 8);
        assert_eq!(g.value_at(2, 2), 9);
                
    }

    #[test]
    fn test_grid3d_creation() {
        let nx = 512;
        let ny = 510;
        let nz = 513;
        let g = Grid3D::<f64>::new(nx, ny, nz);
        assert_eq!(g.dim[0], nx);
        assert_eq!(g.dim[1], ny);
        assert_eq!(g.dim[2], nz);
        assert_eq!(g.data.len(), nx * ny * nz);
    }
    
    #[test]
    fn test_grid3d_idx() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let g = Grid3D::<u32>::from_raw_data(&data, 2, 2, 2).unwrap();
        assert_eq!(g.value_at(0, 0, 0), 1);
        assert_eq!(g.value_at(1, 0, 0), 2);
        assert_eq!(g.value_at(0, 1, 0), 3);
        assert_eq!(g.value_at(1, 1, 0), 4);
        assert_eq!(g.value_at(0, 0, 1), 5);
        assert_eq!(g.value_at(1, 0, 1), 6);
        assert_eq!(g.value_at(0, 1, 1), 7);
        assert_eq!(g.value_at(1, 1, 1), 8);
        
    }

}
