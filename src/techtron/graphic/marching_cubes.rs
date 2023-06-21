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

mod marching_cubes_lut;

use approx::relative_eq;
use marching_cubes_lut::*;

use log::info;

// Vertices:           Faces:
//     0 __________4        ___________
//    /|          /|      /|          /|       ______
//   / |         / |     / |   0     / |      /|     x
// 1/__________5/  |    /  |     3  /  |     / |
// |   |       |   |   |___________| 5 |    y  |
// |   3_______|___7   | 4 |_______|___|       z
// |  /        |  /    |  /  1     |  /
// | /         | /     | /     2   | /
// 2/__________6/      |/__________|/

type Point3D = [f32; 3];
type Normal3D = [f32; 3];
type Index3D = [usize; 3];

#[wasm_bindgen]
pub struct Surface {
    vertex: Vec<Point3D>,
    index: Vec<Index3D>,
    normal: Vec<Normal3D>,
}

impl Surface {
    pub fn new() -> Surface {
        Surface {
            vertex: Vec::new(),
            index: Vec::new(),
            normal: Vec::new(),
        }
    }

    pub fn transform_vertices_mut<F>(&mut self, f: F) 
    where F: Fn(&Point3D) -> Point3D
    {
        let mut vertex = Vec::with_capacity(self.vertex.len());
        for p in &self.vertex {
            vertex.push(f(p));
        }
        self.vertex = vertex;
    }

    // pub fn get_vertex(&mut self) -> &mut Vec<Point3D> {
    //     &mut self.vertex
    // }

    // pub fn get_index(&mut self) -> &mut Vec<Index3D> {
    //     &mut self.index
    // }

    // pub fn get_normal(&mut self) -> &mut Vec<Normal3D> {
    //     &mut self.normal
    // }
}

pub struct MarchingCubesBuilder {
    pub nx: i32,
    pub ny: i32,
    pub nz: i32,
    pub iso: f32,
    pub Dx: Vec<Vec<u32>>,
    pub Dy: Vec<Vec<u32>>,
    pub Ux: Vec<Vec<u32>>,
    pub Uy: Vec<Vec<u32>>,
    pub Lz: Vec<Vec<u32>>,
}

impl MarchingCubesBuilder {
    pub fn new(iso: f32, nx: i32, ny: i32, nz: i32) -> MarchingCubesBuilder {
        let mut Lz: Vec<Vec<u32>> = Vec::with_capacity(ny as usize);
        let mut Dy: Vec<Vec<u32>> = Vec::with_capacity(ny as usize - 1);
        let mut Uy: Vec<Vec<u32>> = Vec::with_capacity(ny as usize - 1);
        let mut Dx: Vec<Vec<u32>> = Vec::with_capacity(ny as usize);
        let mut Ux: Vec<Vec<u32>> = Vec::with_capacity(ny as usize);

        for j in 0..ny as usize - 1 {
            let mut tmp = Vec::with_capacity(nx as usize - 1);
            tmp.resize(nx as usize - 1, 0);
            Dx.push(tmp);

            let mut tmp = Vec::with_capacity(nx as usize - 1);
            tmp.resize(nx as usize - 1, 0);
            Ux.push(tmp);

            let mut tmp = Vec::with_capacity(nx as usize);
            tmp.resize(nx as usize, 0);
            Lz.push(tmp);

            let mut tmp = Vec::with_capacity(nx as usize);
            tmp.resize(nx as usize, 0);
            Dy.push(tmp);

            let mut tmp = Vec::with_capacity(nx as usize);
            tmp.resize(nx as usize, 0);
            Uy.push(tmp);
        }

        let mut tmp = Vec::with_capacity(nx as usize - 1);
        tmp.resize(nx as usize - 1, 0);
        Dx.push(tmp);

        let mut tmp = Vec::with_capacity(nx as usize - 1);
        tmp.resize(nx as usize - 1, 0);
        Ux.push(tmp);

        let mut tmp = Vec::with_capacity(nx as usize);
        tmp.resize(nx as usize, 0);
        Lz.push(tmp);
        MarchingCubesBuilder {
            nx,
            ny,
            nz,
            iso,
            Dx,
            Dy,
            Ux,
            Uy,
            Lz,
        }
    }

    pub fn get_pixel(&self, data: &[f32], x: i32, y: i32, z: i32) -> f32 {
        let index = (self.nx * self.ny * z + self.nx * y + x) as usize;
        data[index]
    }
    fn surfint(
        &mut self,
        data: &[f32],
        surface: &mut Surface,
        x: i32,
        y: i32,
        z: i32,
        r: &mut [f32],
    ) -> u32 {
        r[0] = x as f32;
        r[1] = y as f32;
        r[2] = z as f32;
        if x == 0 {
            r[3] = self.get_pixel(data, 0, y, z) - self.get_pixel(data, 1, y, z);
        } else if x == self.nx - 1 {
            r[3] = self.get_pixel(data, x - 1, y, z) - self.get_pixel(data, x, y, z);
        } else {
            r[3] = 0.5 * (self.get_pixel(data, x - 1, y, z) - self.get_pixel(data, x + 1, y, z));
        }
        if y == 0 {
            r[4] = self.get_pixel(data, x, 0, z) - self.get_pixel(data, x, 1, z);
        } else if y == self.ny - 1 {
            r[4] = self.get_pixel(data, x, y - 1, z) - self.get_pixel(data, x, y, z);
        } else {
            r[4] = 0.5 * (self.get_pixel(data, x, y - 1, z) - self.get_pixel(data, x, y + 1, z));
        }
        if z == 0 {
            r[5] = self.get_pixel(data, x, y, 0) - self.get_pixel(data, x, y, 1);
        } else if z == self.nz - 1 {
            r[5] = self.get_pixel(data, x, y, z - 1) - self.get_pixel(data, x, y, z);
        } else {
            r[5] = 0.5 * (self.get_pixel(data, x, y, z - 1) - self.get_pixel(data, x, y, z + 1));
        }
        return store(surface, r, 13);
    }

    fn find_case(
        &mut self,
        data: &[f32],
        surface: &mut Surface,
        x: i32,
        y: i32,
        z: i32,
        i: u32,
        v: &[f32],
    ) {
        // println!("{:04x}", i);
        // println!("{:?}", v);

        let mut c: u32 = 0;
        let mut m: bool;

        if i & 0x80 != 0 {
            c = MARCHING_CUBES_LUT[(i ^ 0xFF) as usize] as u32;
            m = (c & 0x0800) == 0;
        } else {
            c = MARCHING_CUBES_LUT[i as usize] as u32;
            m = (c & 0x0800) != 0;
        }

        let mut k: u32 = c & 0x07FF;
        let mut pcase: u32 = 0;

        // temp memory for face test
        let mut face: [i32; 6] = [0; 6];

        match c >> 12 {
            0 =>
            // cases 1, 2, 5, 8, 9, 11 and 14
            {
                pcase += k
            }
            1 =>
            // case 3
            {
                pcase += (if (if m { i } else { i ^ 0xFF } & face_test1(k >> 2, v)) != 0 {
                        183 + (k << 1)
                    } else {
                        159 + k
                    });
            }
            2 =>
            // case 4
            {
                pcase += if interior_test(k, 0, v) != 0 {
                    239 + 6 * k
                } else {
                    231 + (k << 1)
                }
            }
            3 =>
            // case 6
            {
                if (if m { i } else { i ^ 0xFF }) & face_test1(k % 6, v) != 0 {
                    pcase += 575 + 5 * k; //6.2
                } else {
                    pcase += if interior_test((k / 6) as u32, 0, v) != 0 {
                        407 + 7 * k
                    } else {
                        335 + 3 * k
                    }; //6.1
                }
            }
            4 =>
            // case 7
            {
                match face_test(&mut face, if m { i } else { i ^ 0xFF }, v) {
                    -3 => pcase += 695 + 3 * k, //7.1
                    -1 =>
                    //7.2
                    {
                        pcase += if face[4] + face[5] < 0 {
                            if face[0] + face[2] < 0 {
                                759
                            } else {
                                799
                            }
                        } else {
                            719
                        } + 5 * k
                    }
                    1 =>
                    //7.3
                    {
                        pcase += if face[4] + face[5] < 0 {
                            983
                        } else {
                            if face[0] + face[2] < 0 {
                                839
                            } else {
                                911
                            }
                        } + 9 * k
                    }
                    _ =>
                    //7.4
                    {
                        pcase += if interior_test(k >> 1, 0, v) != 0 {
                            1095 + 9 * k
                        } else {
                            1055 + 5 * k
                        }
                    }
                }
            }
            5 =>
            // case 10
            {
                match face_test(&mut face, if m { i } else { i ^ 0xFF }, v) {
                    -2 => {
                        if (if k == 2 {
                            interior_test(0, 0, v) != 0
                        } else {
                            interior_test(0, 0, v) != 0
                                || interior_test(if k != 0 { 1 } else { 3 }, 0, v) != 0
                        }) {
                            pcase += 1213 + (k << 3); //10.1.2
                        } else {
                            pcase += 1189 + (k << 2); //10.1.1
                        }
                    }
                    0 =>
                    //10.2
                    {
                        pcase += if face[2 + k as usize] < 0 { 1261 } else { 1285 } + (k << 3)
                    }
                    _ => {
                        if (if k == 2 {
                            interior_test(1, 0, v) != 0
                        } else {
                            interior_test(2, 0, v) != 0
                                || interior_test(if k != 0 { 3 } else { 1 }, 0, v) != 0
                        }) {
                            pcase += 1237 + (k << 3); //10.1.2
                        } else {
                            pcase += 1201 + (k << 2); //10.1.1
                        }
                    }
                }
            }
            6 =>
            // case 12
            {
                match face_test(&mut face, if m { i } else { i ^ 0xFF }, v) {
                    -2 =>
                    //12.1
                    {
                        pcase += if interior_test((0xDA010C >> (k << 1)) & 3, 0, v) != 0 {
                            1453 + (k << 3)
                        } else {
                            1357 + (k << 2)
                        }
                    }
                    0 =>
                    //12.2
                    {
                        pcase += if face[(k >> 1) as usize] < 0 {
                            1645
                        } else {
                            1741
                        } + (k << 3)
                    }
                    _ =>
                    //12.1
                    {
                        pcase += if interior_test((0xA7B7E5 >> (k << 1)) & 3, 0, v) != 0 {
                            1549 + (k << 3)
                        } else {
                            1405 + (k << 2)
                        }
                    }
                }
            }
            _ =>
            //case 13
            {
                match face_test(&mut face, 165, v).abs() {
                    0 => {
                        k = (if face[1] < 0 { 1 } else { 0 } << 1)
                            | if face[5] < 0 { 1 } else { 0 };
                        if face[0] * face[1] == face[5] {
                            //13.4
                            pcase += 2157 + 12 * k;
                        } else {
                            c = interior_test(k, 1, v); // 13.5.1 if c == 0 else 13.5.2
                            pcase += 2285 + if c != 0 { 10 * k - 40 * c } else { 6 * k };
                        }
                    }
                    2 => {
                        //13.3
                        //13.3
                        pcase += 1917
                            + 10 * ((if face[0] < 0 {
                                if face[2] > 0 {
                                    1
                                } else {
                                    0
                                }
                            } else {
                                12 + (if face[2] < 0 { 1 } else { 0 })
                            }) + (if face[1] < 0 {
                                if face[3] < 0 {
                                    1
                                } else {
                                    0
                                }
                            } else {
                                6 + (if face[3] > 0 { 1 } else { 0 })
                            }));
                        if face[4] > 0 {
                            pcase += 30
                        };
                    }
                    4 => {
                        //13.2
                        k = (21 + 11 * face[0] + 4 * face[1] + 3 * face[2] + 2 * face[3] + face[4])
                            as u32;
                        if k >> 4 != 0 {
                            k -= (if k & 32 != 0 { 20 } else { 10 });
                        }
                        pcase += 1845 + 3 * k;
                    }
                    _ =>
                    //13.1
                    {
                        pcase += (1839 + 2 * face[0]) as u32
                    }
                }
            }
        }
        // println!("pcase = {}", MARCHING_CUBES_LUT[pcase as usize + 1]);
        let kk = c >> 12;
        let mut i = i;
        let mut p: [u32; 13] = [0xFFFFFFFF; 13];
        let mut r: [f32; 6] = [0.; 6];
        let mut t: f32 = 0.;
        let mut ti: [u32; 3] = [0; 3]; //for vertex indices of a triangle
        while (i != 0) {
            pcase += 1;
            i = MARCHING_CUBES_LUT[pcase as usize] as u32;
            k = 3;
            while (k != 0) {
                c = i & 0x0F;
                i >>= 4;
                if p[c as usize] == 0xFFFFFFFF {
                    // println!("================c = {}", c);
                    match c {
                        0 => {
                            if z != 0 || x != 0 {
                                p[0] = self.Dy[y as usize][x as usize];
                            } else {
                                if relative_eq!(v[0], 0.) {
                                    if p[3] != 0xFFFFFFFF {
                                        p[0] = p[3];
                                    } else if p[8] != 0xFFFFFFFF {
                                        p[0] = p[8];
                                    } else if y != 0 && v[3] < 0. {
                                        p[0] = self.Lz[y as usize][0];
                                    } else if y != 0 && v[4] < 0. {
                                        p[0] = self.Dx[y as usize][0];
                                    } else if (if y != 0 {
                                        self.iso - self.get_pixel(data, 0, y - 1, 0) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[0] = self.Dy[y as usize - 1][0];
                                    } else {
                                        p[0] = self.surfint(data, surface, 0, y, 0, &mut r);
                                    }
                                } else if relative_eq!(v[1], 0.) {
                                    if p[1] != 0xFFFFFFFF {
                                        p[0] = p[1];
                                    } else if p[9] != 0xFFFFFFFF {
                                        p[0] = p[9];
                                    } else {
                                        p[0] = self.surfint(data, surface, 0, y + 1, 0, &mut r);
                                    }
                                } else {
                                    t = v[0] / (v[0] - v[1]);
                                    r[0] = 0.;
                                    r[2] = 0.;
                                    r[1] = y as f32 + t;
                                    r[3] = (v[4] - v[0]) * (1. - t) + (v[5] - v[1]) * t;
                                    r[4] = v[1] - v[0];
                                    r[5] = (v[3] - v[0]) * (1. - t) + (v[2] - v[1]) * t;
                                    p[0] = store(surface, &r, 0);
                                }
                                self.Dy[y as usize][0] = p[0]
                            }
                        }
                        1 => {
                            if x != 0 {
                                p[1] = self.Lz[y as usize + 1][x as usize];
                            } else {
                                if relative_eq!(v[1], 0.) {
                                    if p[0] != 0xFFFFFFFF {
                                        p[1] = p[0];
                                    } else if p[9] != 0xFFFFFFFF {
                                        p[1] = p[9];
                                    } else if z != 0 && v[0] < 0. {
                                        p[1] = self.Dy[y as usize][0];
                                    } else if z != 0 && v[5] < 0. {
                                        p[1] = self.Dx[y as usize + 1][0];
                                    } else if z != 0
                                        && (if y + 1 < self.ny - 1 {
                                            self.iso - self.get_pixel(data, 0, y + 2, z) < 0.
                                        } else {
                                            false
                                        })
                                    {
                                        p[1] = self.Dy[y as usize + 1][0];
                                    } else if (if z != 0 {
                                        self.iso - self.get_pixel(data, 0, y + 1, z - 1) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[1] = self.Lz[y as usize + 1][0]; // value of previous slice
                                    } else {
                                        p[1] = self.surfint(data, surface, 0, y + 1, z, &mut r);
                                    }
                                } else if relative_eq!(v[2], 0.) {
                                    if p[10] != 0xFFFFFFFF {
                                        p[1] = p[10];
                                    } else if p[2] != 0xFFFFFFFF {
                                        p[1] = p[2];
                                    } else {
                                        p[1] = self.surfint(data, surface, 0, y + 1, z + 1, &mut r);
                                    }
                                } else {
                                    t = v[1] / (v[1] - v[2]);
                                    r[0] = 0.;
                                    r[1] = y as f32 + 1.;
                                    r[2] = z as f32 + t;
                                    r[3] = (v[5] - v[1]) * (1. - t) + (v[6] - v[2]) * t;
                                    r[4] = if y + 1 < self.ny - 1 {
                                        0.5 * ((self.get_pixel(data, 0, y, z)
                                            - self.get_pixel(data, 0, y + 2, z))
                                            * (1. - t)
                                            + (self.get_pixel(data, 0, y, z + 1)
                                                - self.get_pixel(data, 0, y + 2, z + 1))
                                                * t)
                                    } else {
                                        (v[1] - v[0]) * (1. - t) + (v[2] - v[3]) * t
                                    };
                                    r[5] = v[2] - v[1];
                                    p[1] = store(surface, &r, 1);
                                }
                                self.Lz[y as usize + 1][0] = p[1];
                            }
                        }
                        2 => {
                            if x != 0 {
                                p[2] = self.Uy[y as usize][x as usize];
                            } else {
                                if relative_eq!(v[3], 0.) {
                                    if p[3] != 0xFFFFFFFF {
                                        p[2] = p[3];
                                    } else if p[11] != 0xFFFFFFFF {
                                        p[2] = p[11];
                                    } else if y != 0 && v[0] < 0. {
                                        p[2] = self.Lz[y as usize][0];
                                    } else if y != 0 && v[7] < 0. {
                                        p[2] = self.Ux[y as usize][0];
                                    } else if (if y != 0 {
                                        self.iso - self.get_pixel(data, 0, y - 1, z + 1) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[2] = self.Uy[y as usize - 1][0];
                                    } else {
                                        p[2] = self.surfint(data, surface, 0, y, z + 1, &mut r);
                                    }
                                } else if relative_eq!(v[2], 0.) {
                                    if p[10] != 0xFFFFFFFF {
                                        p[2] = p[10];
                                    } else if p[1] != 0xFFFFFFFF {
                                        p[2] = p[1];
                                    } else {
                                        p[2] = self.surfint(data, surface, 0, y + 1, z + 1, &mut r);
                                    }
                                } else {
                                    t = v[3] / (v[3] - v[2]);
                                    r[0] = 0.;
                                    r[2] = z as f32 + 1.;
                                    r[1] = y as f32 + t;
                                    r[3] = (v[7] - v[3]) * (1. - t) + (v[6] - v[2]) * t;
                                    r[4] = v[2] - v[3];
                                    r[5] = if z + 1 < self.nz - 1 {
                                        0.5 * ((self.get_pixel(data, 0, y, z)
                                            - self.get_pixel(data, 0, y, z + 2))
                                            * (1. - t)
                                            + (self.get_pixel(data, 0, y + 1, z)
                                                - self.get_pixel(data, 0, y + 1, z + 2))
                                                * t)
                                    } else {
                                        (v[3] - v[0]) * (1. - t) + (v[2] - v[1]) * t
                                    };
                                    p[2] = store(surface, &r, 2);
                                }
                                self.Uy[y as usize][0] = p[2];
                            }
                        }
                        3 => {
                            if y != 0 || x != 0 {
                                p[3] = self.Lz[y as usize][x as usize];
                            } else {
                                if relative_eq!(v[0], 0.) {
                                    if p[0] != 0xFFFFFFFF {
                                        p[3] = p[0];
                                    } else if p[8] != 0xFFFFFFFF {
                                        p[3] = p[8];
                                    } else if z != 0 && v[1] < 0. {
                                        p[3] = self.Dy[0][0];
                                    } else if z != 0 && v[4] < 0. {
                                        p[3] = self.Dx[0][0];
                                    } else if (if z != 0 {
                                        self.iso - self.get_pixel(data, 0, 0, z - 1) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[3] = self.Lz[0][0]; // value of previous slice
                                    } else {
                                        p[3] = self.surfint(data, surface, 0, 0, z, &mut r);
                                    }
                                } else if relative_eq!(v[3], 0.) {
                                    if p[2] != 0xFFFFFFFF {
                                        p[3] = p[2];
                                    } else if p[11] != 0xFFFFFFFF {
                                        p[3] = p[11];
                                    } else {
                                        p[3] = self.surfint(data, surface, 0, 0, z + 1, &mut r);
                                    }
                                } else {
                                    t = v[0] / (v[0] - v[3]);
                                    r[0] = 0.;
                                    r[1] = 0.;
                                    r[2] = z as f32 + t;
                                    r[3] = (v[4] - v[0]) * (1. - t) + (v[7] - v[3]) * t;
                                    r[4] = (v[1] - v[0]) * (1. - t) + (v[2] - v[3]) * t;
                                    r[5] = v[3] - v[0];
                                    p[3] = store(surface, &r, 3);
                                }
                                self.Lz[0][0] = p[3];
                            }
                        }
                        4 => {
                            if z != 0 {
                                p[4] = self.Dy[y as usize][x as usize + 1];
                                // println!("-------------{}", p[4]);
                            } else {
                                if relative_eq!(v[4], 0.) {
                                    if p[8] != 0xFFFFFFFF {
                                        p[4] = p[8];
                                    } else if y != 0 && v[0] < 0. {
                                        p[4] = self.Dx[y as usize][x as usize];
                                    } else if y != 0 && v[7] < 0. {
                                        p[4] = self.Lz[y as usize][x as usize + 1];
                                    } else if (if y != 0 {
                                        self.iso - self.get_pixel(data, x + 1, y - 1, 0) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[4] = self.Dy[y as usize - 1][x as usize + 1];
                                    } else if (if y != 0 && x + 1 < self.nx - 1 {
                                        self.iso - self.get_pixel(data, x + 2, y, 0) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[4] = self.Dx[y as usize][x as usize + 1];
                                    } else {
                                        p[4] = self.surfint(data, surface, x + 1, y, 0, &mut r);
                                    }
                                } else if relative_eq!(v[5], 0.) {
                                    if p[5] != 0xFFFFFFFF {
                                        p[4] = p[5];
                                    } else if p[9] != 0xFFFFFFFF {
                                        p[4] = p[9];
                                    } else {
                                        p[4] = self.surfint(data, surface, x + 1, y + 1, 0, &mut r);
                                    }
                                } else {
                                    t = v[4] / (v[4] - v[5]);
                                    r[0] = x as f32 + 1.;
                                    r[2] = 0.;
                                    r[1] = y as f32 + t;
                                    r[3] = if x + 1 < self.nx - 1 {
                                        0.5 * ((self.get_pixel(data, x, y, 0)
                                            - self.get_pixel(data, x + 2, y, 0))
                                            * (1. - t)
                                            + (self.get_pixel(data, x, y + 1, 0)
                                                - self.get_pixel(data, x + 2, y + 1, 0))
                                                * t)
                                    } else {
                                        (v[4] - v[0]) * (1. - t) + (v[5] - v[1]) * t
                                    };
                                    r[4] = v[5] - v[4];
                                    r[5] = (v[7] - v[4]) * (1. - t) + (v[6] - v[5]) * t;
                                    p[4] = store(surface, &r, 4);
                                }
                                self.Dy[y as usize][x as usize + 1] = p[4];
                            }
                        }
                        5 => {
                            if relative_eq!(v[5], 0.) {
                                if z != 0 {
                                    if v[4] < 0. {
                                        p[4] = self.Dy[y as usize][x as usize + 1];
                                        p[5] = p[4];
                                    } else if v[1] < 0. {
                                        p[9] = self.Dx[y as usize + 1][x as usize];
                                        p[5] = p[9];
                                    } else if (if x + 1 < self.nx - 1 {
                                        self.iso - self.get_pixel(data, x + 2, y + 1, z) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[5] = self.Dx[y as usize + 1][x as usize + 1];
                                    } else if (if y + 1 < self.ny - 1 {
                                        self.iso - self.get_pixel(data, x + 1, y + 2, z) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[5] = self.Dy[y as usize + 1][x as usize + 1];
                                    } else if self.iso - self.get_pixel(data, x + 1, y + 1, z - 1)
                                        < 0.
                                    {
                                        p[5] = self.Lz[y as usize + 1][x as usize + 1];
                                    // value of previous slice
                                    } else {
                                        p[5] = self.surfint(data, surface, x + 1, y + 1, z, &mut r);
                                    }
                                } else {
                                    p[5] = self.surfint(data, surface, x + 1, y + 1, 0, &mut r);
                                }
                            } else if relative_eq!(v[6], 0.) {
                                p[5] = self.surfint(data, surface, x + 1, y + 1, z + 1, &mut r);
                            } else {
                                t = v[5] / (v[5] - v[6]);
                                r[0] = x as f32 + 1.;
                                r[1] = y as f32 + 1.;
                                r[2] = z as f32 + t;
                                // if r[2] < 0. {
                                //     info!("+++=== x: {} y: {} z: {} t: {} {:?}", x, y, z, t, v);
                                //     info!("+++=== v[5]: {} v[5] - v[6] {} ", v[5], v[5] - v[6]);
                                //     info!("+++=== case: {} ", kk);
                                // } 
                                r[3] = if x + 1 < self.nx - 1 {
                                    0.5 * ((self.get_pixel(data, x, y + 1, z)
                                        - self.get_pixel(data, x + 2, y + 1, z))
                                        * (1. - t)
                                        + (self.get_pixel(data, x, y + 1, z + 1)
                                            - self.get_pixel(data, x + 2, y + 1, z + 1))
                                            * t)
                                } else {
                                    (v[5] - v[1]) * (1. - t) + (v[6] - v[2]) * t
                                };
                                r[4] = if y + 1 < self.ny - 1 {
                                    0.5 * ((self.get_pixel(data, x + 1, y, z)
                                        - self.get_pixel(data, x + 1, y + 2, z))
                                        * (1. - t)
                                        + (self.get_pixel(data, x + 1, y, z + 1)
                                            - self.get_pixel(data, x + 1, y + 2, z + 1))
                                            * t)
                                } else {
                                    (v[5] - v[4]) * (1. - t) + (v[6] - v[7]) * t
                                };
                                r[5] = v[6] - v[5];
                                p[5] = store(surface, &r, 5);
                            }
                            self.Lz[y as usize + 1][x as usize + 1] = p[5];
                        }
                        6 => {
                            if relative_eq!(v[7], 0.) {
                                if y != 0 {
                                    if v[3] < 0. {
                                        p[11] = self.Ux[y as usize][x as usize];
                                        p[6] = p[11];
                                    } else if v[4] < 0. {
                                        p[7] = self.Lz[y as usize][x as usize + 1];
                                        p[6] = p[7];
                                    } else if self.iso - self.get_pixel(data, x + 1, y - 1, z + 1)
                                        < 0.
                                    {
                                        p[6] = self.Uy[y as usize - 1][x as usize + 1];
                                    } else if (if x + 1 < self.nx - 1 {
                                        self.iso - self.get_pixel(data, x + 2, y, z + 1) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[6] = self.Ux[y as usize][x as usize + 1];
                                    } else {
                                        p[6] = self.surfint(data, surface, x + 1, y, z + 1, &mut r);
                                    }
                                } else if p[11] != 0xFFFFFFFF {
                                    p[6] = p[11];
                                } else {
                                    p[6] = self.surfint(data, surface, x + 1, 0, z + 1, &mut r);
                                }
                            } else if relative_eq!(v[6], 0.) {
                                if p[5] == 0xFFFFFFFF {
                                    p[6] = if p[10] == 0xFFFFFFFF {
                                        self.surfint(data, surface, x + 1, y + 1, z + 1, &mut r)
                                    } else {
                                        p[10]
                                    };
                                } else {
                                    p[6] = p[5];
                                }
                            } else {
                                t = v[7] / (v[7] - v[6]);
                                r[0] = x as f32 + 1.;
                                r[1] = y as f32 + t;
                                // if r[1] < 0. {
                                //     info!("+++=== x: {} y: {} z: {} t: {} {:?}", x, y, z, t, v);
                                //     info!("+++=== v[7]: {} v[7] - v[6] {} ", v[7], v[7] - v[6]);
                                //     info!("+++=== case: {} ", kk);
                                // } 
                                r[2] = z as f32 + 1.;
                                r[3] = if x + 1 < self.nx - 1 {
                                    0.5 * ((self.get_pixel(data, x, y, z + 1)
                                        - self.get_pixel(data, x + 2, y, z + 1))
                                        * (1. - t)
                                        + (self.get_pixel(data, x, y + 1, z + 1)
                                            - self.get_pixel(data, x + 2, y + 1, z + 1))
                                            * t)
                                } else {
                                    (v[7] - v[3]) * (1. - t) + (v[6] - v[2]) * t
                                };
                                r[4] = v[6] - v[7];
                                r[5] = if z + 1 < self.nz - 1 {
                                    0.5 * ((self.get_pixel(data, x + 1, y, z)
                                        - self.get_pixel(data, x + 1, y, z + 2))
                                        * (1. - t)
                                        + (self.get_pixel(data, x + 1, y + 1, z)
                                            - self.get_pixel(data, x + 1, y + 1, z + 2))
                                            * t)
                                } else {
                                    (v[7] - v[4]) * (1. - t) + (v[6] - v[5]) * t
                                };
                                p[6] = store(surface, &r, 6);
                            }
                            self.Uy[y as usize][x as usize + 1] = p[6];
                        }
                        7 => {
                            if y != 0 {
                                p[7] = self.Lz[y as usize][x as usize + 1];
                            } else {
                                if relative_eq!(v[4], 0.) {
                                    if (p[8] != 0xFFFFFFFF) {
                                        p[7] = p[8];
                                    } else if (p[4] != 0xFFFFFFFF) {
                                        p[7] = p[4];
                                    } else if z != 0 && v[0] < 0. {
                                        p[7] = self.Dx[0][x as usize];
                                    } else if z != 0 && v[5] < 0. {
                                        p[7] = self.Dy[0][x as usize + 1];
                                    } else if (if z != 0 && x + 1 < self.nx - 1 {
                                        self.iso - self.get_pixel(data, x + 2, 0, z) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[7] = self.Dx[0][x as usize + 1];
                                    } else if (if z != 0 {
                                        self.iso - self.get_pixel(data, x + 1, 0, z - 1) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[7] = self.Lz[0][x as usize + 1]; // value of previous slice
                                    } else {
                                        p[7] = self.surfint(data, surface, x + 1, 0, z, &mut r);
                                    }
                                } else if relative_eq!(v[7], 0.) {
                                    if (p[6] != 0xFFFFFFFF) {
                                        p[7] = p[6];
                                    } else if (p[11] != 0xFFFFFFFF) {
                                        p[7] = p[11];
                                    } else {
                                        p[7] = self.surfint(data, surface, x + 1, 0, z + 1, &mut r);
                                    }
                                } else {
                                    t = v[4] / (v[4] - v[7]);
                                    r[0] = x as f32 + 1.;
                                    r[1] = 0.;
                                    r[2] = z as f32 + t;
                                    r[3] = if x + 1 < self.nx - 1 {
                                        0.5 * ((self.get_pixel(data, x, 0, z)
                                            - self.get_pixel(data, x + 2, 0, z))
                                            * (1. - t)
                                            + (self.get_pixel(data, x, 0, z + 1)
                                                - self.get_pixel(data, x + 2, 0, z + 1))
                                                * t)
                                    } else {
                                        (v[4] - v[0]) * (1. - t) + (v[7] - v[3]) * t
                                    };
                                    r[4] = (v[5] - v[4]) * (1. - t) + (v[6] - v[7]) * t;
                                    r[5] = v[7] - v[4];
                                    p[7] = store(surface, &r, 7);
                                }
                                self.Lz[0][x as usize + 1] = p[7];
                            }
                        }
                        8 => {
                            if z != 0 || y != 0 {
                                p[8] = self.Dx[y as usize][x as usize];
                            } else {
                                if relative_eq!(v[0], 0.) {
                                    if (p[3] != 0xFFFFFFFF) {
                                        p[8] = p[3];
                                    } else if (p[0] != 0xFFFFFFFF) {
                                        p[8] = p[0];
                                    } else if x != 0 && v[3] < 0. {
                                        p[8] = self.Lz[0][x as usize];
                                    } else if x != 0 && v[1] < 0. {
                                        p[8] = self.Dy[0][x as usize];
                                    } else if (if x != 0 {
                                        self.iso - self.get_pixel(data, x - 1, 0, 0) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[8] = self.Dx[0][x as usize - 1];
                                    } else {
                                        p[8] = self.surfint(data, surface, x, 0, 0, &mut r);
                                    }
                                } else if relative_eq!(v[4], 0.) {
                                    if (p[4] != 0xFFFFFFFF) {
                                        p[8] = p[4];
                                    } else if (p[7] != 0xFFFFFFFF) {
                                        p[8] = p[7];
                                    } else {
                                        p[8] = self.surfint(data, surface, x + 1, 0, 0, &mut r);
                                    }
                                } else {
                                    t = v[0] / (v[0] - v[4]);
                                    r[1] = 0.;
                                    r[2] = 0.;
                                    r[0] = x as f32 + t;
                                    r[3] = v[4] - v[0];
                                    r[4] = (v[1] - v[0]) * (1. - t) + (v[5] - v[4]) * t;
                                    r[5] = (v[3] - v[0]) * (1. - t) + (v[7] - v[4]) * t;
                                    p[8] = store(surface, &r, 8);
                                }
                                self.Dx[0][x as usize] = p[8];
                            }
                        }
                        9 => {
                            if z != 0 {
                                p[9] = self.Dx[y as usize + 1][x as usize];
                            } else {
                                if relative_eq!(v[1], 0.) {
                                    if (p[0] != 0xFFFFFFFF) {
                                        p[9] = p[0];
                                    } else if x != 0 && v[0] < 0. {
                                        p[9] = self.Dy[y as usize][x as usize];
                                    } else if x != 0 && v[2] < 0. {
                                        p[9] = self.Lz[y as usize + 1][x as usize];
                                    } else if (if x != 0 {
                                        self.iso - self.get_pixel(data, x - 1, y + 1, 0) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[9] = self.Dx[y as usize + 1][x as usize - 1];
                                    } else {
                                        p[9] = self.surfint(data, surface, x, y + 1, 0, &mut r);
                                    }
                                } else if relative_eq!(v[5], 0.) {
                                    if (p[5] != 0xFFFFFFFF) {
                                        p[9] = p[5];
                                    } else if (p[4] != 0xFFFFFFFF) {
                                        p[9] = p[4];
                                    } else {
                                        p[9] = self.surfint(data, surface, x + 1, y + 1, 0, &mut r);
                                    }
                                } else {
                                    t = v[1] / (v[1] - v[5]);
                                    r[1] = y as f32 + 1.;
                                    r[2] = 0.;
                                    r[0] = x as f32 + t;
                                    r[3] = v[5] - v[1];
                                    r[4] = if y + 1 < self.ny - 1 {
                                        0.5 * ((self.get_pixel(data, x, y, 0)
                                            - self.get_pixel(data, x, y + 2, 0))
                                            * (1. - t)
                                            + (self.get_pixel(data, x + 1, y, 0)
                                                - self.get_pixel(data, x + 1, y + 2, 0))
                                                * t)
                                    } else {
                                        (v[1] - v[0]) * (1. - t) + (v[5] - v[4]) * t
                                    };
                                    r[5] = (v[2] - v[1]) * (1. - t) + (v[6] - v[5]) * t;
                                    p[9] = store(surface, &r, 9);
                                }
                                self.Dx[y as usize + 1][x as usize] = p[9];
                            }
                        }
                        10 => {
                            if relative_eq!(v[2], 0.) {
                                if x != 0 {
                                    if v[1] < 0. {
                                        p[1] = self.Lz[y as usize + 1][x as usize];
                                        p[10] = p[1];
                                    } else if v[3] < 0. {
                                        p[2] = self.Uy[y as usize][x as usize];
                                        p[10] = p[2];
                                    } else if self.iso - self.get_pixel(data, x - 1, y + 1, z + 1)
                                        < 0.
                                    {
                                        p[10] = self.Ux[y as usize + 1][x as usize - 1];
                                    } else {
                                        p[10] =
                                            self.surfint(data, surface, x, y + 1, z + 1, &mut r);
                                    }
                                } else if (p[2] != 0xFFFFFFFF) {
                                    p[10] = p[2];
                                } else {
                                    p[10] = self.surfint(data, surface, 0, y + 1, z + 1, &mut r);
                                }
                            } else if relative_eq!(v[6], 0.) {
                                if (p[5] == 0xFFFFFFFF) {
                                    p[10] = (if p[6] == 0xFFFFFFFF {
                                        self.surfint(data, surface, x + 1, y + 1, z + 1, &mut r)
                                    } else {
                                        p[6]
                                    });
                                } else {
                                    p[10] = p[5];
                                }
                            } else {
                                t = v[2] / (v[2] - v[6]);
                                r[0] = x as f32 + t;
                                r[1] = y as f32 + 1.;
                                r[2] = z as f32 + 1.;
                                r[3] = v[6] - v[2];
                                r[4] = if y + 1 < self.ny - 1 {
                                    0.5 * ((self.get_pixel(data, x, y, z + 1)
                                        - self.get_pixel(data, x, y + 2, z + 1))
                                        * (1. - t)
                                        + (self.get_pixel(data, x + 1, y, z + 1)
                                            - self.get_pixel(data, x + 1, y + 2, z + 1))
                                            * t)
                                } else {
                                    (v[2] - v[3]) * (1. - t) + (v[6] - v[7]) * t
                                };
                                r[5] = if z + 1 < self.nz - 1 {
                                    0.5 * ((self.get_pixel(data, x, y + 1, z)
                                        - self.get_pixel(data, x, y + 1, z + 2))
                                        * (1. - t)
                                        + (self.get_pixel(data, x + 1, y + 1, z)
                                            - self.get_pixel(data, x + 1, y + 1, z + 2))
                                            * t)
                                } else {
                                    (v[2] - v[1]) * (1. - t) + (v[6] - v[5]) * t
                                };
                                p[10] = store(surface, &r, 10);
                            }
                            self.Ux[y as usize + 1][x as usize] = p[10];
                        }
                        11 => {
                            if y != 0 {
                                p[11] = self.Ux[y as usize][x as usize];
                            } else {
                                if relative_eq!(v[3], 0.) {
                                    if (p[3] != 0xFFFFFFFF) {
                                        p[11] = p[3];
                                    } else if (p[2] != 0xFFFFFFFF) {
                                        p[11] = p[2];
                                    } else if x != 0 && v[0] < 0. {
                                        p[11] = self.Lz[0][x as usize];
                                    } else if x != 0 && v[2] < 0. {
                                        p[11] = self.Uy[0][x as usize];
                                    } else if (if x != 0 {
                                        self.iso - self.get_pixel(data, x - 1, 0, z + 1) < 0.
                                    } else {
                                        false
                                    }) {
                                        p[11] = self.Ux[0][x as usize - 1];
                                    } else {
                                        p[11] = self.surfint(data, surface, x, 0, z + 1, &mut r);
                                    }
                                } else if relative_eq!(v[7], 0.) {
                                    if (p[6] != 0xFFFFFFFF) {
                                        p[11] = p[6];
                                    } else if (p[7] != 0xFFFFFFFF) {
                                        p[11] = p[7];
                                    } else {
                                        p[11] =
                                            self.surfint(data, surface, x + 1, 0, z + 1, &mut r);
                                    }
                                } else {
                                    t = v[3] / (v[3] - v[7]);
                                    r[1] = 0.;
                                    r[2] = z as f32 + 1.;
                                    r[0] = x as f32 + t;
                                    r[3] = v[7] - v[3];
                                    r[4] = (v[2] - v[3]) * (1. - t) + (v[6] - v[7]) * t;
                                    r[5] = if z + 1 < self.nz - 1 {
                                        0.5 * ((self.get_pixel(data, x, 0, z)
                                            - self.get_pixel(data, x, 0, z + 2))
                                            * (1. - t)
                                            + (self.get_pixel(data, x + 1, 0, z)
                                                - self.get_pixel(data, x + 1, 0, z + 2))
                                                * t)
                                    } else {
                                        (v[3] - v[0]) * (1. - t) + (v[7] - v[4]) * t
                                    };
                                    p[11] = store(surface, &r, 11);
                                }
                                self.Ux[0][x as usize] = p[11];
                            }
                        }
                        _ => {
                            r[0] = x as f32 + 0.5;
                            r[1] = y as f32 + 0.5;
                            r[2] = z as f32 + 0.5;
                            r[3] = v[4] + v[5] + v[6] + v[7] - v[0] - v[1] - v[2] - v[3];
                            r[4] = v[1] + v[2] + v[5] + v[6] - v[0] - v[3] - v[4] - v[7];
                            r[5] = v[2] + v[3] + v[6] + v[7] - v[0] - v[1] - v[4] - v[5];
                            p[12] = store(surface, &r, 12);
                        }
                    }
                }
                k -= 1;
                ti[k as usize] = p[c as usize]; //now ti contains the vertex indices of the triangle
                                                // println!("p[{}] = {}", c, p[c as usize]);
            }
            if (ti[0] != ti[1] && ti[0] != ti[2] && ti[1] != ti[2])
            //to avoid zero area triangles
            {
                let idx0 = ti[if m { 0 } else { 1 }] as usize;
                let idx1 = ti[if m { 1 } else { 0 }] as usize;
                let idx2 = ti[2] as usize;
                surface.index.push([idx0, idx1, idx2]);
                // println!("index: {:?}", (ti[if m {0} else {1}], ti[if m {1} else {0}], ti[2]));
            }
        }
    }
}

fn next_z(index: usize, nx: i32, ny: i32, nz: i32) -> usize {
    index + (nx * ny) as usize
}

fn next_y(index: usize, nx: i32, ny: i32, nz: i32) -> usize {
    index + nx as usize
}

fn print_data(data: &[f32], v00: usize, v01: usize, v11: usize, v10: usize) {
    println!(
        "{} {} {} {} {} {} {} {}",
        data[v00],
        data[v01],
        data[v11],
        data[v10],
        data[v00 + 1],
        data[v01 + 1],
        data[v11 + 1],
        data[v10 + 1]
    );
}

fn face_test(face: &mut [i32], ind: u32, v: &[f32]) -> i32 {
    if ind & 0x80 != 0
    //vertex 0
    {
        face[0] = if (ind & 0xCC) == 0x84 {
            if v[0] * v[5] < v[1] * v[4] {
                -1
            } else {
                1
            }
        } else {
            0
        }; //0x84 = 10000100, vertices 0 and 5
        face[3] = if (ind & 0x99) == 0x81 {
            if v[0] * v[7] < v[3] * v[4] {
                -1
            } else {
                1
            }
        } else {
            0
        }; //0x81 = 10000001, vertices 0 and 7
        face[4] = if (ind & 0xF0) == 0xA0 {
            if v[0] * v[2] < v[1] * v[3] {
                -1
            } else {
                1
            }
        } else {
            0
        }; //0xA0 = 10100000, vertices 0 and 2
    } else {
        face[0] = if (ind & 0xCC) == 0x48 {
            if v[0] * v[5] < v[1] * v[4] {
                1
            } else {
                -1
            }
        } else {
            0
        }; //0x48 = 01001000, vertices 1 and 4
        face[3] = if (ind & 0x99) == 0x18 {
            if v[0] * v[7] < v[3] * v[4] {
                1
            } else {
                -1
            }
        } else {
            0
        }; //0x18 = 00011000, vertices 3 and 4
        face[4] = if (ind & 0xF0) == 0x50 {
            if v[0] * v[2] < v[1] * v[3] {
                1
            } else {
                -1
            }
        } else {
            0
        }; //0x50 = 01010000, vertices 1 and 3
    }
    if ind & 0x02 != 0
    //vertex 6
    {
        face[1] = if (ind & 0x66) == 0x42 {
            if v[1] * v[6] < v[2] * v[5] {
                -1
            } else {
                1
            }
        } else {
            0
        }; //0x42 = 01000010, vertices 1 and 6
        face[2] = if (ind & 0x33) == 0x12 {
            if v[3] * v[6] < v[2] * v[7] {
                -1
            } else {
                1
            }
        } else {
            0
        }; //0x12 = 00010010, vertices 3 and 6
        face[5] = if (ind & 0x0F) == 0x0A {
            if v[4] * v[6] < v[5] * v[7] {
                -1
            } else {
                1
            }
        } else {
            0
        }; //0x0A = 00001010, vertices 4 and 6
    } else {
        face[1] = if (ind & 0x66) == 0x24 {
            if v[1] * v[6] < v[2] * v[5] {
                1
            } else {
                -1
            }
        } else {
            0
        }; //0x24 = 00100100, vertices 2 and 5
        face[2] = if (ind & 0x33) == 0x21 {
            if v[3] * v[6] < v[2] * v[7] {
                1
            } else {
                -1
            }
        } else {
            0
        }; //0x21 = 00100001, vertices 2 and 7
        face[5] = if (ind & 0x0F) == 0x05 {
            if v[4] * v[6] < v[5] * v[7] {
                1
            } else {
                -1
            }
        } else {
            0
        }; //0x05 = 00000101, vertices 5 and 7
    }
    return face[0] + face[1] + face[2] + face[3] + face[4] + face[5];
}

fn face_test1(face: u32, v: &[f32]) -> u32 {
    match face {
        0 => {
            if v[0] * v[5] < v[1] * v[4] {
                0x48
            } else {
                0x84
            }
        }
        1 => {
            if v[1] * v[6] < v[2] * v[5] {
                0x24
            } else {
                0x42
            }
        }
        2 => {
            if v[3] * v[6] < v[2] * v[7] {
                0x21
            } else {
                0x12
            }
        }
        3 => {
            if v[0] * v[7] < v[3] * v[4] {
                0x18
            } else {
                0x81
            }
        }
        4 => {
            if v[0] * v[2] < v[1] * v[3] {
                0x50
            } else {
                0xA0
            }
        }
        _ => {
            if v[4] * v[6] < v[5] * v[7] {
                0x05
            } else {
                0x0A
            }
        }
    }
}

fn interior_test(i: u32, flag13: u32, v: &[f32]) -> u32 {
    let mut at = v[4] - v[0];
    let mut bt = v[5] - v[1];
    let mut ct = v[6] - v[2];
    let mut dt = v[7] - v[3];
    let mut t = at * ct - bt * dt; // the "a" value.

    if t < 0. {
        if i & 0x01 != 0 {
            return 0;
        }
    } else {
        if i & 0x01 == 0 || relative_eq!(t, 0.) {
            return 0;
        }
    }
    t = 0.5 * (v[3] * bt - v[2] * at + v[1] * dt - v[0] * ct) / t; // t = -b/2a
    if t > 0. && t < 1. {
        at = v[0] + at * t;
        bt = v[1] + bt * t;
        ct = v[2] + ct * t;
        dt = v[3] + dt * t;
        ct *= at;
        dt *= bt;
        if i & 0x01 != 0 {
            if ct < dt && dt > 0. {
                return if (bt <= 0. && v[i as usize] <= 0.) || (bt >= 0. && v[i as usize] >= 0.) {
                    1
                } else {
                    0
                } + flag13;
            }
        } else {
            if ct > dt && ct > 0. {
                return if (at <= 0. && v[i as usize] <= 0.) || (at >= 0. && v[i as usize] >= 0.) {
                    1
                } else {
                    0
                } + flag13;
            }
        }
    }
    return 0;
}

fn store(surface: &mut Surface, r: &[f32], x: u32) -> u32 {
    surface.vertex.push([r[0], r[1], r[2]]);

    if (r[0] < 0. || r[1] < 0. || r[2] < 0.) {
        info!("+++=== warning: {} {} {} {}", x, r[0], r[1], r[2]);
    }

    let t = 1.0 / (r[3] * r[3] + r[4] * r[4] + r[5] * r[5]).sqrt();
    // println!("vertex: {:?} normal: {:?}", &r[0..3], [t * r[3], t * r[4], t * r[5]]);
    surface.normal.push([t * r[3], t * r[4], t * r[5]]);
    return (surface.vertex.len() - 1) as u32;
}

pub fn marching_cubes_impl(data: &[f32], isovalue: f32, nx: i32, ny: i32, nz: i32) -> Surface {
    let mut builder = MarchingCubesBuilder::new(isovalue, nx, ny, nz);
    let mut surface = Surface::new();

    // It's meaningless if nx/ny/nz is less than 2
    assert!(nx >= 2 && ny >= 2 && nz >= 2);

    // check the size
    assert_eq!(data.len() as i32, nx * ny * nz);

    // index to the data
    let mut p: usize = 0;
    let mut p0: usize = p;
    let mut p1: usize = p;

    // vertices
    let mut v00 = p;
    let mut v01 = p;
    let mut v11 = p;
    let mut v10 = p;

    // temp mem for storing (iso - value)
    // reuse values on the y-z plane
    let mut vt: [f32; 12] = [0.0; 12];
    let mut v1: usize = 0;
    let mut v2: usize = 4;

    // iterate through the grids
    for z in 0..nz - 1 {
        p0 = p;
        p = next_z(p, nx, ny, nz);
        p1 = p;
        for y in 0..ny - 1 {
            v00 = p0;
            p0 = next_y(p0, nx, ny, nz);
            v01 = p0;
            v10 = p1;
            p1 = next_y(p1, nx, ny, nz);
            v11 = p1;

            vt[v2 + 0] = isovalue - data[v00];
            vt[v2 + 1] = isovalue - data[v01];
            vt[v2 + 2] = isovalue - data[v11];
            vt[v2 + 3] = isovalue - data[v10];

            //the eight least significant bits of i correspond to vertex indices. (x...x01234567)
            //If the bit is 1 then the vertex value is greater than zero.
            let mut i: u32 = if vt[v2 + 3] < 0. { 1 } else { 0 };
            if vt[v2 + 2] < 0. {
                i |= 2
            };
            if vt[v2 + 1] < 0. {
                i |= 4
            };
            if vt[v2 + 0] < 0. {
                i |= 8
            };
            // println!("{:b}", i);
            // println!("{:b}", (i & 0x0F) << 4);

            for x in 0..nx - 1 {
                // print_data(data, v00, v01, v11, v10);
                v00 += 1;
                v01 += 1;
                v11 += 1;
                v10 += 1;

                std::mem::swap(&mut v1, &mut v2);
                vt[v2 + 0] = isovalue - data[v00];
                vt[v2 + 1] = isovalue - data[v01];
                vt[v2 + 2] = isovalue - data[v11];
                vt[v2 + 3] = isovalue - data[v10];

                i = ((i & 0x0F) << 4) | if vt[v2 + 3] < 0. { 1 } else { 0 };
                if vt[v2 + 2] < 0. {
                    i |= 2
                };
                if vt[v2 + 1] < 0. {
                    i |= 4
                };
                if vt[v2 + 0] < 0. {
                    i |= 8
                };

                if i != 0 && i ^ 0xFF != 0 {
                    if v1 > v2 {
                        let mut t = v2;
                        let mut s = t + 8;
                        vt[s] = vt[t];
                        s += 1;
                        t += 1;
                        vt[s] = vt[t];
                        s += 1;
                        t += 1;
                        vt[s] = vt[t];
                        s += 1;
                        t += 1;
                        vt[s] = vt[t];
                    }
                    builder.find_case(data, &mut surface, x, y, z, i, &vt[v1..v1 + 8]);
                }
            }
        }
        std::mem::swap(&mut builder.Dx, &mut builder.Ux);
        std::mem::swap(&mut builder.Dy, &mut builder.Uy);
    }
    // for idx in surface.index {
    //     println!("v0: {:?} n0: {:?}", surface.vertex[idx[0]], surface.normal[idx[0]]);
    //     println!("v1: {:?} n1: {:?}", surface.vertex[idx[1]], surface.normal[idx[1]]);
    //     println!("v2: {:?} n2: {:?}", surface.vertex[idx[2]], surface.normal[idx[2]]);
    //     println!();
    // }
    surface
}

#[cfg(test)]
mod test {
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    use super::*;
    fn f1(x: f32, y: f32, z: f32) -> f32 {
        let v = ((x * y).sin() + (y * z).sin() + (x * z).sin()) / (1.0 + x * x + y * y + z * z);
        // println!("({}, {}, {}) {}\n", x, y, z, v);
        return v;
    }

    #[test]
    fn test_marching_cube() {
        let mut data = Vec::<f32>::new();
        for z in (0..3).map(|x| x as f32 * 2.0 - 2.0) {
            for y in (0..3).map(|x| x as f32 * 2.0 - 2.0) {
                for x in (0..3).map(|x| x as f32 * 2.0 - 2.0) {
                    data.push(f1(x, y, z));
                }
            }
        }
        let surface = marching_cubes_impl(&data, 0.02, 3, 3, 3);
        let vertex: Vec<[f32; 3]> = vec![
            [2.000, 1.000, 0.762],
            [2.000, 0.762, 1.000],
            [1.238, 1.000, 0.000],
            [1.238, 0.000, 1.000],
            [1.731, 0.000, 0.000],
            [1.000, 2.000, 0.762],
            [0.000, 1.238, 1.000],
            [0.762, 2.000, 1.000],
            [1.000, 1.238, 0.000],
            [0.000, 1.731, 0.000],
            [2.000, 2.000, 0.269],
            [1.000, 0.762, 2.000],
            [0.762, 1.000, 2.000],
            [1.000, 0.000, 1.238],
            [0.000, 1.000, 1.238],
            [0.000, 0.000, 1.731],
            [2.000, 0.269, 2.000],
            [0.269, 2.000, 2.000],
        ];
        assert_eq!(surface.vertex.len(), vertex.len());
        for v in 0..vertex.len() {
            for i in 0..3 {
                assert_abs_diff_eq!(vertex[v][i], surface.vertex[v][i], epsilon = 0.001);
            }
        }
        let normal: Vec<[f32; 3]> = vec![
            [-0.186, 0.596, 0.781],
            [-0.186, 0.781, 0.596],
            [-0.781, -0.596, 0.186],
            [-0.781, 0.186, -0.596],
            [-0.924, -0.270, -0.270],
            [0.596, -0.186, 0.781],
            [0.186, -0.781, -0.596],
            [0.781, -0.186, 0.596],
            [-0.596, -0.781, 0.186],
            [-0.270, -0.924, -0.270],
            [0.270, 0.270, 0.924],
            [0.596, 0.781, -0.186],
            [0.781, 0.596, -0.186],
            [-0.596, 0.186, -0.781],
            [0.183, -0.614, -0.768],
            [-0.270, -0.270, -0.924],
            [0.270, 0.924, 0.270],
            [0.924, 0.270, 0.270],
        ];
        assert_eq!(surface.normal.len(), normal.len());
        for v in 0..normal.len() {
            for i in 0..3 {
                assert_abs_diff_eq!(normal[v][i], surface.normal[v][i], epsilon = 0.001);
            }
        }
        let index: Vec<[usize; 3]> = vec![
            [2, 1, 0],
            [2, 3, 1],
            [4, 3, 2],
            [7, 6, 5],
            [6, 8, 5],
            [9, 8, 6],
            [5, 0, 10],
            [0, 8, 2],
            [0, 5, 8],
            [13, 12, 11],
            [13, 14, 12],
            [15, 14, 13],
            [1, 11, 16],
            [3, 13, 11],
            [11, 1, 3],
            [12, 7, 17],
            [7, 14, 6],
            [7, 12, 14],
        ];
        assert_eq!(index, surface.index);
    }
}

// WASM
use js_sys::{Float32Array, Int32Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn marching_cubes(
    data: &[f32],
    isovalue: f32,
    nx: i32,
    ny: i32,
    nz: i32,
) -> Result<Surface, String> {
    let surface = marching_cubes_impl(data, isovalue, nx, ny, nz);
    Ok(surface)
}

#[wasm_bindgen]
impl Surface {
    pub fn get_vertices(&self) -> Result<Float32Array, String> {
        // every index got 3 points and every points got 3 f32
        let num_of_values = self.index.len() * 3 * 3;
        let mut vertices: Vec<f32> = Vec::with_capacity(num_of_values);
        for idx in &self.index {
            for i in 0..3 {
                for j in 0..3 {
                    vertices.push(self.vertex[idx[i]][j]);
                }
            }
            // println!("v0: {:?} n0: {:?}", self.vertex[idx[0]], self.normal[idx[0]]);
            // println!("v1: {:?} n1: {:?}", self.vertex[idx[1]], self.normal[idx[1]]);
            // println!("v2: {:?} n2: {:?}", self.vertex[idx[2]], self.normal[idx[2]]);
            // println!();
        }
        let mut rtv = Float32Array::new_with_length(num_of_values as u32);
        rtv.copy_from(&vertices);
        Ok(rtv)
    }

    pub fn get_normals(&self) -> Result<Float32Array, String> {
        // every index got 3 points and every points got 3 f32
        let num_of_values = self.index.len() * 3 * 3;
        let mut normals: Vec<f32> = Vec::with_capacity(num_of_values);
        for idx in &self.index {
            for i in 0..3 {
                for j in 0..3 {
                    normals.push(self.normal[idx[i]][j]);
                }
            }
        }
        let mut rtv = Float32Array::new_with_length(num_of_values as u32);
        rtv.copy_from(&normals);
        Ok(rtv)
    }
}
