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


// const LUT: &str = r#""#;

pub const LUT: [u8; 256 * 3] = [
    // ncolors = 256
    // # r   g   b
    4, 35, 51, 4, 36, 53, 4, 37, 55, 4, 37, 57, 5, 38, 59, 5, 39, 61, 5, 39, 63, 5, 40, 65, 5, 41,
    67, 6, 41, 69, 6, 42, 71, 6, 43, 73, 7, 43, 75, 7, 44, 77, 7, 44, 80, 8, 45, 82, 8, 46, 84, 9,
    46, 86, 9, 47, 89, 10, 47, 91, 11, 48, 93, 12, 48, 96, 12, 48, 98, 13, 49, 101, 14, 49, 103,
    15, 50, 106, 16, 50, 108, 18, 50, 111, 19, 51, 114, 20, 51, 116, 22, 51, 119, 23, 51, 122, 25,
    51, 124, 26, 52, 127, 28, 52, 130, 30, 52, 132, 31, 52, 135, 33, 52, 138, 35, 52, 140, 37, 52,
    143, 39, 52, 145, 42, 51, 147, 44, 51, 149, 46, 51, 151, 48, 51, 153, 51, 51, 155, 53, 51, 156,
    55, 51, 157, 57, 51, 158, 60, 51, 159, 62, 52, 159, 64, 52, 159, 66, 52, 160, 68, 53, 160, 70,
    53, 160, 71, 54, 160, 73, 54, 159, 75, 55, 159, 77, 55, 159, 78, 56, 158, 80, 57, 158, 82, 57,
    157, 83, 58, 157, 85, 59, 157, 86, 59, 156, 88, 60, 156, 89, 61, 155, 91, 61, 155, 92, 62, 154,
    94, 63, 154, 95, 63, 153, 96, 64, 153, 98, 65, 152, 99, 65, 152, 101, 66, 151, 102, 67, 151,
    103, 67, 150, 105, 68, 150, 106, 69, 149, 108, 69, 149, 109, 70, 148, 110, 71, 148, 112, 71,
    148, 113, 72, 147, 114, 72, 147, 116, 73, 146, 117, 74, 146, 118, 74, 146, 120, 75, 145, 121,
    75, 145, 122, 76, 145, 124, 77, 144, 125, 77, 144, 126, 78, 144, 128, 78, 143, 129, 79, 143,
    131, 80, 143, 132, 80, 142, 133, 81, 142, 135, 81, 142, 136, 82, 141, 137, 82, 141, 139, 83,
    141, 140, 83, 140, 142, 84, 140, 143, 84, 140, 144, 85, 139, 146, 85, 139, 147, 86, 139, 149,
    86, 138, 150, 87, 138, 151, 87, 138, 153, 88, 137, 154, 88, 137, 156, 89, 137, 157, 89, 136,
    159, 90, 136, 160, 90, 135, 162, 91, 135, 163, 91, 134, 165, 92, 134, 166, 92, 134, 168, 93,
    133, 169, 93, 132, 171, 93, 132, 172, 94, 131, 174, 94, 131, 175, 95, 130, 177, 95, 130, 178,
    96, 129, 180, 96, 128, 181, 97, 128, 183, 97, 127, 184, 98, 126, 186, 98, 126, 187, 98, 125,
    189, 99, 124, 190, 99, 123, 192, 100, 123, 193, 100, 122, 195, 101, 121, 196, 101, 120, 198,
    102, 119, 199, 102, 118, 201, 103, 117, 202, 103, 116, 204, 104, 115, 205, 104, 114, 206, 105,
    113, 208, 105, 112, 209, 106, 111, 211, 106, 110, 212, 107, 109, 214, 108, 108, 215, 108, 107,
    216, 109, 106, 218, 110, 105, 219, 110, 104, 220, 111, 102, 222, 112, 101, 223, 112, 100, 224,
    113, 99, 225, 114, 98, 227, 114, 96, 228, 115, 95, 229, 116, 94, 230, 117, 93, 231, 118, 91,
    232, 119, 90, 234, 120, 89, 235, 121, 88, 236, 121, 86, 237, 122, 85, 238, 123, 84, 238, 125,
    83, 239, 126, 82, 240, 127, 80, 241, 128, 79, 242, 129, 78, 243, 130, 77, 243, 131, 76, 244,
    133, 75, 245, 134, 74, 245, 135, 73, 246, 136, 72, 246, 138, 71, 247, 139, 70, 247, 140, 69,
    248, 142, 68, 248, 143, 67, 249, 145, 67, 249, 146, 66, 249, 147, 65, 250, 149, 65, 250, 150,
    64, 250, 152, 63, 251, 153, 63, 251, 155, 62, 251, 156, 62, 251, 158, 62, 251, 159, 61, 251,
    161, 61, 252, 163, 61, 252, 164, 61, 252, 166, 60, 252, 167, 60, 252, 169, 60, 252, 170, 60,
    252, 172, 60, 252, 174, 60, 252, 175, 60, 252, 177, 60, 251, 178, 61, 251, 180, 61, 251, 182,
    61, 251, 183, 61, 251, 185, 62, 251, 187, 62, 251, 188, 62, 250, 190, 63, 250, 191, 63, 250,
    193, 64, 250, 195, 64, 249, 196, 65, 249, 198, 65, 249, 200, 66, 248, 201, 67, 248, 203, 67,
    248, 205, 68, 247, 206, 69, 247, 208, 69, 247, 210, 70, 246, 211, 71, 246, 213, 71, 245, 215,
    72, 245, 216, 73, 244, 218, 74, 244, 220, 75, 243, 221, 75, 243, 223, 76, 242, 225, 77, 242,
    226, 78, 241, 228, 79, 241, 230, 80, 240, 232, 81, 239, 233, 81, 239, 235, 82, 238, 237, 83,
    237, 238, 84, 237, 240, 85, 236, 242, 86, 235, 244, 87, 234, 245, 88, 234, 247, 89, 233, 249,
    90, 232, 250, 91,
];
