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


// Bresenham Line algorithm


pub fn generate_line2d(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<[i32;2]> {
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 {1} else {-1};
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 {1} else {-1};

    let mut line_points = Vec::new();
    let mut error = dx + dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        line_points.push([x, y]);

        if x == x2 && y == y2 {break;}
        let e2 = 2 * error;
        if e2 >= dy {
            if x == x2 {break;}
            error += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y2 {break;}
            error += dx;
            y += sy;
        }
    }
    return line_points;
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bresenham_line() {
        let line = generate_line2d(-2, -2, 2, 2);
        println!("{:?}", line);

        let line = generate_line2d(2, 2, -2, -2);
        println!("{:?}", line);

        let line = generate_line2d(-5, -2, 2, 10);
        println!("{:?}", line);
    }
}