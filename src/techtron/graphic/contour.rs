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


use std::cmp;

use js_sys::Array;
use log::info;
use num::Num;
use wasm_bindgen::prelude::*;

use crate::techtron::{
    core::grid::Grid2D,
    graphic::{line::generate_line2d, marching_cubes::marching_cubes_impl, Point, lerp},
};

use super::{
    marching_cubes::Surface, LineSegment, LineSegmentInPlace, LineSegments, Point2D,
    PointLineSegmentRelation,
};

#[derive(Debug, Clone)]
pub struct Contour<T = f32, const N: usize = 2>
where
    T: Copy + Num + cmp::PartialOrd,
{
    data: LineSegments<T, N>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Contour2Df32 {
    data: Vec<Point2D<f32>>,
}

#[wasm_bindgen]
impl Contour2Df32 {
    #[cfg(target_family = "wasm")]
    pub fn get_points(&self) -> Result<Array, String> {
        let mut v = Array::new();
        let last_idx = self.data.len();
        for i in 0..last_idx {
            let point = &self.data[i];
            v.push(&point.to_array());
        }
        Ok(v)
    }
}

impl Contour2Df32 {
    pub fn transform_points<F>(&self, f: F) -> Contour2Df32
    where
        F: Fn(&Point2D<f32>) -> Point2D<f32>,
    {
        let mut data = Vec::with_capacity(self.data.len());
        for p in &self.data {
            data.push(f(p));
        }
        Contour2Df32 { data }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Contour2Df32Builder {
    // data: LineSegments<f32, 2>,
    data: Vec<Point2D<f32>>,
}

#[wasm_bindgen]
impl Contour2Df32Builder {
    pub fn new() -> Contour2Df32Builder {
        Contour2Df32Builder {
            data: Vec::<Point2D<f32>>::new(),
        }
    }

    pub fn test_push(&self, x: f32, y: f32) -> bool {
        let p0 = Point2D::<f32>::new([x, y]);
        let num_of_points = self.data.len();
        if num_of_points <= 1 {
            return true;
        }
        let last = self.data.len() - 1;
        let last_line_segment =
            LineSegment::new(self.data[last].clone(), self.data[last - 1].clone());

        match p0.relative_to(&last_line_segment, 3.) {
            // web_sys::console::log_1(&JsValue::from("1"));
            // info!("1");
            PointLineSegmentRelation::OnRayAB => {
                // web_sys::console::log_1(&JsValue::from(format!("on the RayAB {:?} {:?}", &last_line_segment, &p0)));
                return false;
            }
            PointLineSegmentRelation::On => {
                // web_sys::console::log_1(&JsValue::from(format!("on the line {:?} {:?}", &last_line_segment, &p0)));
                return false;
            }
            _ => (),
        }
        // web_sys::console::log_1(&JsValue::from(format!("on the line {:?} {:?}", &last_line_segment, &p0)));
        // return false;
        // } else {
        //     web_sys::console::log_1(&JsValue::from(format!("not on the line {:?} {:?}", &last_line_segment, &p0)));
        // }

        let line_segment0 = LineSegmentInPlace::<f32, 2>::new(&self.data[last], &p0);
        for i in 0..last - 1 {
            let line_segment1 = LineSegmentInPlace::<f32, 2>::new(&self.data[i], &self.data[i + 1]);
            #[cfg(target_family = "wasm")]
            web_sys::console::log_1(&JsValue::from(format!(
                "{:?} {:?}",
                &line_segment0, &line_segment1
            )));
            // web_sys::console::log_1(&JsValue::from("."));
            if line_segment0.is_intersect(&line_segment1, 0.1) {
                // web_sys::console::log_1(&JsValue::from("2"));
                // web_sys::console::log_1(&JsValue::from(format!("{:?} {:?}", &line_segment0, &line_segment1)));
                // info!("2");
                return false;
            }
        }
        return true;
    }

    pub fn push(&mut self, x: f32, y: f32) {
        self.data.push(Point2D::<f32>::new([x, y]));
    }

    pub fn test_close(&self) -> bool {
        let num_of_points = self.data.len();

        // case 1: 2 points
        if num_of_points <= 2 {
            return false;
        }

        // case 2: 3 points
        if num_of_points == 3 {
            return true;
        }

        // case 3: >= 4 points
        let last = self.data.len() - 1;
        let line_segment = LineSegment::new(self.data[last].clone(), self.data[0].clone());
        if self.data[1].is_on(&line_segment, 0.5) || self.data[last - 1].is_on(&line_segment, 0.5) {
            return false;
        }

        let line_segment0 = LineSegmentInPlace::<f32, 2>::new(&self.data[0], &self.data[last]);
        for i in 1..last - 1 {
            let line_segment1 = LineSegmentInPlace::<f32, 2>::new(&self.data[i], &self.data[i + 1]);
            if line_segment0.is_intersect(&line_segment1, 0.5) {
                return false;
            }
        }
        return true;
    }

    pub fn close(self) -> Contour2Df32 {
        Contour2Df32 { data: self.data }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Structure {
    contours: Vec<Contour2Df32>,
    z: Vec<f32>,
}

// #[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl Structure {
    pub fn new() -> Structure {
        Structure {
            contours: Vec::new(),
            z: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.contours.len()
    }

    pub fn push(&mut self, z: f32, contour: Contour2Df32) {
        self.z.push(z);
        self.contours.push(contour);
    }

    #[cfg(target_family = "wasm")]
    pub fn get_contours_at(&self, z: f32) -> Result<JsValue, String> {
        if let Some(contours) = self.get_contours_at_impl(z) {
            let arr = Array::new();
            for c in contours {
                let points = c.get_points().unwrap();
                arr.push(&points);
            }
            return Ok(JsValue::from(arr));
        }
        Ok(JsValue::undefined())
    }

    pub fn to_mesh(&self) -> Surface {
        let [xmin, xmax, ymin, ymax, zmin, zmax] = self.bounding_box();
        // const W: usize = 250;
        // const H: usize = 250;
        let W = (xmax - xmin + 1.).max(50.) as usize;
        let H = (ymax - ymin + 1.).max(50.) as usize;
        let mut sorted_z = Vec::new();
        for (i, z) in self.z.iter().enumerate() {
            sorted_z.push((z, i));
        }
        sorted_z.sort_by(|(z0, _), (z1, _)| z0.partial_cmp(z1).unwrap());
        let mut data: Vec<f32> = Vec::new();
        for (z, i) in &sorted_z {
            let mut accumulator: Grid2D<u8> = Grid2D::new(W, H);
            let contours = self.get_contours_at_impl(**z).unwrap();
            for contour in contours {
                let c = contour.clone().transform_points(|&p| {
                    // scale to [1, W-2]
                    let x = (p[0] - xmin) / (xmax - xmin) * (W - 1 - 1) as f32 + 1.0;
                    let y = (p[1] - ymin) / (ymax - ymin) * (H - 1 - 1) as f32 + 1.0;
                    Point2D::<f32>::new([x, y])
                });
                accumulator = fill_contour(accumulator, &c);
            }
            // for y in 0..H {
            //     let mut s = String::new();
            //     for x in 0..W {
            //         let v = accumulator.value_at(x, y);
            //         s = format!("{}{}", s, v);
            //     }
            //     info!("{}", s);
            // }
            for v in accumulator.data() {
                data.push(*v as f32);
            }
            info!("");
        }
        let mut surface = marching_cubes_impl(&data, 0.5, W as i32, H as i32, self.z.len() as i32);
        surface.transform_vertices_mut(|&p| {
             let x = (p[0] - 1.0) * (xmax - xmin) / (W - 1 - 1) as f32 + xmin;
            let y = (p[1] - 1.0) * (ymax - ymin) / (H - 1 - 1) as f32 + ymin;
            let z0: i32 = p[2].floor() as i32;
            let z1: i32 = p[2].ceil() as i32;
            let z = lerp(*sorted_z[z0 as usize].0, *sorted_z[z1 as usize].0, p[2] - z0 as f32);
            [x, y, z]
        });
        return surface;
    }
}

impl Structure {
    #[cfg(not(target_family = "wasm"))]
    pub fn get_contours_at<'a>(&'a self, z: f32) -> Option<Vec<&'a Contour2Df32>> {
        self.get_contours_at_impl(z)
    }

    fn get_contours_at_impl<'a>(&'a self, z: f32) -> Option<Vec<&'a Contour2Df32>> {
        let mut ret: Vec<&Contour2Df32> = Vec::new();
        let it = self
            .z
            .iter()
            .enumerate()
            .filter(|(i, &v)| approx::abs_diff_eq!(v, z))
            .for_each(|(i, v)| {
                ret.push(&self.contours[i]);
            });
        if ret.is_empty() {
            return None;
        }
        Some(ret)
    }

    // return value: [xmin, xmax, ymin, ymax, zmin, zmax]
    pub fn bounding_box(&self) -> [f32; 6] {
        assert_eq!(self.z.is_empty(), false);

        // find z min max
        let mut z_min = f32::MAX;
        let mut z_max = f32::MIN;
        for v in &self.z {
            if *v > z_max {
                z_max = *v;
            } else if *v < z_min {
                z_min = *v;
            }
        }

        // find x, y min max
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;

        for z in &self.z {
            let contours = self.get_contours_at_impl(*z).unwrap();
            for contour in &contours {
                for p in &contour.data {
                    if p[0] > x_max {
                        x_max = p[0];
                    } 
                    if p[0] < x_min {
                        x_min = p[0];
                    }
                    if p[1] > y_max {
                        y_max = p[1];
                    } 
                    if p[1] < y_min {
                        y_min = p[1];
                    }
                }
            }
        }
        [x_min, x_max, y_min, y_max, z_min, z_max]
    }

    pub fn calc_vol(&self) -> f32 {
        todo!()
    }
}

#[wasm_bindgen]
struct StructureSet {
    structures: Vec<Structure>,
}

pub fn fill_contour(mut accumulator: Grid2D<u8>, contour: &Contour2Df32) -> Grid2D<u8> {
    let [width, height] = accumulator.dim();
    let first_point = [*contour.data.first().unwrap()];
    let ys = contour.data[1..].iter().chain(first_point.iter());
    let point_paires = contour.data.iter().zip(ys);

    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    for p in point_paires {
        let x0 = p.0[0] as i32;
        let y0 = p.0[1] as i32;
        let x1 = p.1[0] as i32;
        let y1 = p.1[1] as i32;
        // info!("x0: {} y0: {} x1： {} y1: {}", x0, y0, x1, y1);
        let polygon = if x0 < x1 {
            generate_line2d(x0, y0, x1, y1)
        } else {
            generate_line2d(x1, y1, x0, y0)
        };
        let mut yt = i32::MIN;
        for [x, y] in polygon {
            let mut v = accumulator.value_at(x as usize, y as usize);
            if y0 != y1 && y != y1.min(y0) && y != yt {
                v += 1;
            } else {
                v += 2;
            }
            accumulator.set_value_at(x as usize, y as usize, &v);
            yt = y;
        }
        
        let update_min = |min: &mut i32, a: i32, b: i32| {
            let t = a.min(b);
            if t < *min {
                *min = t;
            } 
        };
        let update_max = |max: &mut i32, a: i32, b: i32| {
            let t = a.max(b);
            if t > *max {
                *max = t;
        }
        };
        
        update_min(&mut xmin, x0, x1);
        update_min(&mut ymin, y0, y1);
        update_max(&mut xmax, x0, x1);
        update_max(&mut ymax, y0, y1);
    }

    // scan line
    enum S {
        Inside,
        Outside,
    }
    use S::*;
    for y in ymin as usize..=ymax as usize {
        let mut k = Outside;
        for x in xmin as usize..=xmax as usize {
            let v = accumulator.value_at(x, y);
            match k {
                Outside => {
                    if v % 2 == 1 {
                        k = Inside;
                    }
                    if v != 0 {
                        accumulator.set_value_at(x, y, &1);
                    }
                },
                Inside => {
                    if v % 2 == 1 {
                        k = Outside;
                    } 
                    accumulator.set_value_at(x, y, &1);
                },
            }
        }
    }
    
    for y in 0..height {
        let mut s = String::from("");
        for x in 0..width {
            print!("{}", accumulator.value_at(x, y));
            s.push(accumulator.value_at(x, y).into());
        }
        println!();
        info!("{}", s);
    }
    return accumulator;
}

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    use super::*;

    #[test]
    fn test_approx() {
        assert_abs_diff_eq!(1., 1.01, epsilon = 0.1);
        assert_abs_diff_eq!(1.01, 1., epsilon = 0.1);
        assert_relative_eq!(0., 0.01, epsilon = 0.1);
        let s = Structure::new();
        assert_eq!(0, s.len());
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_structure() {
        let mut s = Structure::new();
        let mut cb = Contour2Df32Builder::new();
        cb.push(2., 10.);
        cb.push(3., 2.);
        cb.push(4., 5.);
        cb.push(6., 1.);
        cb.push(10., 1.);
        cb.push(20., 2.);
        cb.push(9., 9.);
        cb.push(3., 9.);
        let contour = cb.close();
        let mut accumulator = Grid2D::<u8>::new(50, 30);
        fill_contour(accumulator, &contour);
        s.push(1.24, contour);
        assert_eq!(true, s.get_contours_at(1.23).is_none());
        assert_eq!(true, s.get_contours_at(1.24).is_some());

        let mut s = Structure::new();
        let mut cb = Contour2Df32Builder::new();
        cb.push(2., 10.);
        cb.push(13., 0.);
        cb.push(20., 5.);
        cb.push(45., 20.);
        cb.push(10., 9.);
        let contour = cb.close();
        let mut accumulator = Grid2D::<u8>::new(50, 30);
        fill_contour(accumulator, &contour);
        s.push(1.24, contour);
        assert_eq!(true, s.get_contours_at(1.23).is_none());
        assert_eq!(true, s.get_contours_at(1.24).is_some());
    }

    #[test]
    fn test_to_mesh() {
        let mut s = Structure::new();
        let mut cb = Contour2Df32Builder::new();
        cb.push(2., 30.);
        cb.push(13., 20.);
        cb.push(20., 25.);
        cb.push(45., 40.);
        cb.push(10., 29.);
        let contour = cb.close();
        s.push(1.24, contour);

        let mut cb = Contour2Df32Builder::new();
        cb.push(2., 10.);
        cb.push(3., 2.);
        cb.push(4., 5.);
        cb.push(6., 1.);
        cb.push(10., 1.);
        cb.push(20., 2.);
        cb.push(9., 9.);
        cb.push(3., 9.);
        let contour = cb.close();

        s.push(1.24, contour);

        // let mut cb= Contour2Df32Builder::new();
        // cb.push(2., 10.);
        // cb.push(13., 0.);
        // cb.push(20., 5.);
        // cb.push(45., 20.);
        // cb.push(10., 9.);
        // let contour = cb.close();
        // s.push(0.24, contour);

        s.to_mesh();
    }

    #[test]
    fn test() {
        let v: i8 = -1;
        println!("{}", v % 2);
    }
}
