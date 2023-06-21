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



pub mod marching_squares;
pub mod marching_cubes;
pub mod line;
pub mod contour;

use std::{cmp, collections::HashMap, ops::Index};

use approx::relative_eq;
use nalgebra::partial_max;
use num::traits::{real::Real, Num, Pow, Zero};

// #[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_family = "wasm")]
use js_sys::{Array, ArrayBuffer, Float64Array};

#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;

use crate::console;

#[derive(Debug, Clone, Copy, cmp::PartialEq)]
pub struct Point<T = f32, const N: usize = 2>
where
    T: Num + cmp::PartialOrd + Copy,
{
    data: [T; N],
}

impl<T, const N: usize> Point<T, N>
where
    T: Num + cmp::PartialOrd + Copy,
{
    pub fn new(data: [T; N]) -> Point<T, N> {
        Point { data }
    }
}

#[cfg(target_family = "wasm")]
impl<T, const N: usize> Point<T, N>
where
    T: Num + cmp::PartialOrd + Copy + Into<f64>,
{
    pub fn to_array(&self) -> Array {
        let mut v = Array::new();
        for i in 0..N {
            v.push(&JsValue::from(Into::<f64>::into(self[i])));
        }
        v
    }
}

pub enum PointLineSegmentRelation {
    On,
    OnRayAB,
    OnRayBA,
    Appart,
}

impl<T> Point<T, 2>
where
    T: num::Num + cmp::PartialOrd + Copy + Into<f64> + approx::AbsDiffEq,
{
    pub fn relative_to(
        &self,
        line_segment: &LineSegment<T, 2>,
        epsilon: f64,
    ) -> PointLineSegmentRelation {
        let x0 = line_segment.p0[0];
        let y0 = line_segment.p0[1];
        let x1 = line_segment.p1[0];
        let y1 = line_segment.p1[1];
        let dx = x0 - x1;
        let dy = y0 - y1;

        if dx == T::zero() {
            // parallel to y axis
            if approx::abs_diff_eq!(self[0], x0) {
                if y0 < y1 {
                    if self[1] > y1 {
                        return PointLineSegmentRelation::OnRayAB;
                    } else if self[1] < y0 {
                        return PointLineSegmentRelation::OnRayBA;
                    } else {
                        return PointLineSegmentRelation::On;
                    }
                } else if y0 > y1 {
                    if self[1] > y0 {
                        return PointLineSegmentRelation::OnRayBA;
                    } else if self[1] < y1 {
                        return PointLineSegmentRelation::OnRayAB;
                    } else {
                        return PointLineSegmentRelation::On;
                    }
                } else {
                    unreachable!()
                }
            }
        }

        if dy == T::zero() {
            // parallel to x axis
            if approx::abs_diff_eq!(self[1], y0) {
                if x0 < x1 {
                    if self[0] > x1 {
                        return PointLineSegmentRelation::OnRayAB;
                    } else if self[0] < x0 {
                        return PointLineSegmentRelation::OnRayBA;
                    } else {
                        return PointLineSegmentRelation::On;
                    }
                } else if x0 > x1 {
                    if self[0] > x0 {
                        return PointLineSegmentRelation::OnRayBA;
                    } else if self[0] < x1 {
                        return PointLineSegmentRelation::OnRayAB;
                    } else {
                        return PointLineSegmentRelation::On;
                    }
                } else {
                    unreachable!()
                }
            }
        }

        // now k shall not be zero
        //           dy
        // y = y0 + ---- * (x - x0)
        //           dx
        let y: f64 = y0.into() + dy.into() / dx.into() * (self[0].into() - x0.into());
        if approx::abs_diff_eq!(y, self[1].into(), epsilon = epsilon) {
            if x0 < x1 {
                if self[0] > x1 {
                    return PointLineSegmentRelation::OnRayAB;
                } else if self[0] < x0 {
                    return PointLineSegmentRelation::OnRayBA;
                } else {
                    return PointLineSegmentRelation::On;
                }
            } else if x0 > x1 {
                if self[0] > x0 {
                    return PointLineSegmentRelation::OnRayBA;
                } else if self[0] < x1 {
                    return PointLineSegmentRelation::OnRayAB;
                } else {
                    return PointLineSegmentRelation::On;
                }
            } else {
                unreachable!()
            }
        }
        return PointLineSegmentRelation::Appart;
    }

    pub fn is_on(&self, line_segment: &LineSegment<T, 2>, epsilon: f64) -> bool {
        if let PointLineSegmentRelation::On = self.relative_to(line_segment, epsilon) {
            return true;
        }
        return false;
    }
}

type Point2D<T> = Point<T, 2>;
type Point3D<T> = Point<T, 3>;

impl<T, const N: usize> Index<usize> for Point<T, N>
where
    T: Num + cmp::PartialOrd + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

struct PointInPlace<'a, T = f32, const N: usize = 2>
where
    T: Num + cmp::PartialOrd + Copy,
{
    data: &'a [T; N],
}

impl<'a, T, const N: usize> PointInPlace<'a, T, N>
where
    T: Num + cmp::PartialOrd + Copy,
{
    pub fn new(data: &'a [T; N]) -> PointInPlace<'a, T, N> {
        PointInPlace { data }
    }
}

impl<'a, T, const N: usize> Index<usize> for PointInPlace<'a, T, N>
where
    T: Num + cmp::PartialOrd + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[derive(Debug, Clone, Copy, cmp::PartialEq)]
pub struct LineSegment<T = f32, const N: usize = 2>
where
    T: Num + cmp::PartialOrd + Copy,
{
    p0: Point<T, N>,
    p1: Point<T, N>,
}

impl<T, const N: usize> LineSegment<T, N>
where
    T: Num + cmp::PartialOrd + Copy,
{
    pub fn new(p0: Point<T, N>, p1: Point<T, N>) -> LineSegment<T, N> {
        LineSegment { p0, p1 }
    }
}

impl<T, const N: usize> LineSegment<T, N>
where
    T: Num + cmp::PartialOrd + Copy + Into<f64>,
{
    #[cfg(target_family = "wasm")]
    pub fn to_array(&self) -> Array {
        let mut v = Array::new();
        v.push(&self.p0.to_array());
        v.push(&self.p1.to_array());
        v
    }
}

fn is_intersect_2d_impl<T>(p0: &[T; 2], p1: &[T; 2], p2: &[T; 2], p3: &[T; 2], epsilon: f64) -> bool
where
    T: num::Num + cmp::PartialOrd + Copy + Into<f64> + approx::AbsDiffEq,
{
    let x0 = p0[0];
    let y0 = p0[1];
    let x1 = p1[0];
    let y1 = p1[1];
    let x2 = p2[0];
    let y2 = p2[1];
    let x3 = p3[0];
    let y3 = p3[1];
    let dx0 = x0 - x1;
    let dy0 = y0 - y1;
    let dx1 = x2 - x3;
    let dy1 = y2 - y3;

    if approx::abs_diff_eq!(p0[0], p1[0]) && approx::abs_diff_eq!(p2[0], p3[0]) {
        // parallel to y axis
        if approx::abs_diff_ne!(x0, x2) {
            return false;
        } else {
            return !(partial_ord_max(y0, y1) < partial_ord_min(y2, y3)
                || partial_ord_min(y0, y1) > partial_ord_max(y2, y3));
        }
    }

    if approx::abs_diff_eq!(p0[1], p1[1]) && approx::abs_diff_eq!(p2[1], p3[1]) {
        // parallel to x axis
        return !(partial_ord_max(x0, x1) < partial_ord_min(x2, x3)
            || partial_ord_min(x0, x1) > partial_ord_max(x2, x3));
    }

    let y0_: f64 = y2.into() + dy1.into() / dx1.into() * (x0.into() - x2.into());
    let y1_: f64 = y2.into() + dy1.into() / dx1.into() * (x1.into() - x2.into());
    let y2_: f64 = y0.into() + dy0.into() / dx0.into() * (x2.into() - x0.into());
    let y3_: f64 = y0.into() + dy0.into() / dx0.into() * (x3.into() - x0.into());
    if (y0.into() > y0_ && y1.into() > y1_) || (y0.into() < y0_ && y1.into() < y1_) {
        return false;
    }
    if (y2.into() > y2_ && y3.into() > y3_) || (y2.into() < y2_ && y3.into() < y3_) {
        return false;
    }
    return true;
}

impl<T> LineSegment<T, 2>
where
    T: num::Num + cmp::PartialOrd + Copy + Into<f64> + approx::AbsDiffEq,
{
    pub fn is_intersect(&self, rhs: &LineSegment<T, 2>, epsilon: f64) -> bool {
        is_intersect_2d_impl(
            &self.p0.data,
            &self.p1.data,
            &rhs.p0.data,
            &rhs.p1.data,
            epsilon,
        )
    }
}

fn partial_ord_max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

fn partial_ord_min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Debug)]
struct LineSegmentInPlace<'a, T = f32, const N: usize = 2>
where
    T: Num + cmp::PartialOrd + Copy,
{
    p0: &'a Point<T, N>,
    p1: &'a Point<T, N>,
}

impl<'a, T, const N: usize> LineSegmentInPlace<'a, T, N>
where
    T: Num + cmp::PartialOrd + Copy,
{
    pub fn new(p0: &'a Point<T, N>, p1: &'a Point<T, N>) -> LineSegmentInPlace<'a, T, N> {
        LineSegmentInPlace { p0, p1 }
    }
}

impl<'a, T> LineSegmentInPlace<'a, T, 2>
where
    T: Num + cmp::PartialOrd + Copy + Into<f64> + approx::AbsDiffEq,
{
    pub fn is_intersect(&self, rhs: &LineSegmentInPlace<'a, T, 2>, epsilon: f64) -> bool {
        is_intersect_2d_impl(
            &self.p0.data,
            &self.p1.data,
            &rhs.p0.data,
            &rhs.p1.data,
            epsilon,
        )
    }
}

pub trait Distance {
    type Output;
    fn distance(&self, rhs: &Self) -> Self::Output;
}

impl<T, const N: usize> Distance for Point<T, N>
where
    T: Real,
{
    type Output = T;
    fn distance(&self, rhs: &Self) -> Self::Output {
        let sum: T = T::zero();
        for i in 0..N {
            sum == sum + (self[i] - rhs[i]).powi(2);
        }
        sum.sqrt()
    }
}

struct Base<T = f32, const N: usize = 2> {
    // Row major
    data: [[T; N]; N],
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    if relative_eq!(t, 0.) || t < 0. {
        return a;
    } else if relative_eq!(t, 1.) || t > 1. {
        return b;
    } else {
        a + (b - a) * t
    }
}

#[derive(Debug, Clone)]
pub struct LineSegments<T = f32, const N: usize = 2>
where
    T: Copy + Num + cmp::PartialOrd,
{
    data: Vec<LineSegment<T, N>>,
}

impl<T, const N: usize> LineSegments<T, N>
where
    T: Copy + Num + cmp::PartialOrd,
{
    pub fn new() -> LineSegments<T, N> {
        LineSegments {
            data: Vec::<LineSegment<T, N>>::new(),
        }
    }

    pub fn push(&mut self, line_segment: LineSegment<T, N>) {
        self.data.push(line_segment);
    }
}


#[cfg(test)]
mod test {
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    use super::*;
    macro_rules! setup_line_segment {
        ($p0: expr, $p1: expr, $p2: expr, $p3: expr, $epsilon: expr, $t: expr) => {
            let p0 = Point2D::new($p0);
            let p1 = Point2D::new($p1);
            let p2 = Point2D::new($p2);
            let p3 = Point2D::new($p3);
            let line_segment0 = LineSegment::new(p0, p1);
            let line_segment1 = LineSegment::new(p2, p3);
            assert_eq!(line_segment0.is_intersect(&line_segment1, $epsilon), $t);
        };
    }
    #[test]
    fn test_line_segments() {
        let p0 = Point2D::new([0, 0]);
        // let p0 = Point2D::<i32>::new([0, 0]);
        let p1 = Point2D::<i32>::new([30, 30]);
        let p2 = Point2D::<i32>::new([10, 15]);
        let p3 = Point2D::<i32>::new([20, 21]);
        let line_segment0 = LineSegment::new(p0, p1);
        assert_eq!(p2.is_on(&line_segment0, 0.5), false);

        let line_segment1 = LineSegment::new(p2, p3);
        assert_eq!(line_segment0.is_intersect(&line_segment1, 0.5), false);

        let p0 = Point2D::<f32>::new([0., 0.]);
        let p1 = Point2D::<f32>::new([30., 30.]);
        let p2 = Point2D::<f32>::new([15., 15.]);
        let line_segment0 = LineSegment::new(p0, p1);
        assert_eq!(p2.is_on(&line_segment0, 0.5), true);

        setup_line_segment!([0, 0], [30, 30], [10, 15], [20, 21], 0.5, false);
        setup_line_segment!([0., 0.], [30., 30.], [10., 15.], [20., 19.], 0.5, true);
        setup_line_segment!([-10., -15.], [30., 25.], [10., 5.], [10., 19.], 0.1, true);
        setup_line_segment!(
            [-10., -15.],
            [30., 25.],
            [10., 5.01],
            [10., 109.],
            0.001,
            false
        );
        setup_line_segment!([10., 0.], [10., 30.], [10., 30.001], [10., 50.], 0.5, false);
        setup_line_segment!([10., 0.], [10., 30.], [10., 15.], [10., 19.], 0.5, true);

        let p0 = Point2D::<f32>::new([1., 0.]);
        let p1 = Point2D::<f32>::new([1., 30.]);
        let p2 = Point2D::<f32>::new([1., 15.]);
        let line_segment0 = LineSegment::new(p0, p1);
        assert_eq!(p2.is_on(&line_segment0, 0.5), true);

        let p0 = Point2D::<f32>::new([1., 0.]);
        let p1 = Point2D::<f32>::new([1., 30.]);
        let p2 = Point2D::<f32>::new([1., 30.001]);
        let line_segment0 = LineSegment::new(p0, p1);
        assert_eq!(p2.is_on(&line_segment0, 0.5), false);
    }


    

}
