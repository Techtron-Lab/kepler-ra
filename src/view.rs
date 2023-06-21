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


use std::cell::RefCell;
use std::rc::Rc;

use log::info;
use web_sys::WebGl2RenderingContext;

use crate::colormap;
use crate::techtron::prelude::*;

type GL2 = WebGl2RenderingContext;
pub trait Rendable {
    fn render(&mut self);
}

pub trait UpdateUniforms {
    fn update_uniforms(&self, program: &Program) -> Result<(), String>;
}

pub trait BindTextureUniforms {
    fn bind_texture_uniforms(&self, program: &Program) -> Result<(), String>;
}

pub struct View {
    pub program: Rc<RefCell<Program>>,
    pub texture: Rc<RefCell<LoadedTexture>>,
}

impl View {
    pub fn new(
        context: &GLContext,
        program: Rc<RefCell<Program>>,
        texture: Rc<RefCell<LoadedTexture>>,
    ) -> Result<Self, String> {
        {
            // Have to be in a block so that the borrow lasts only until the end
            // of the scope.
            let program_context = &program.borrow().context;
            let texture_context = &texture.borrow().context;
            if context != program_context || program_context != texture_context {
                return Err(String::from(
                    "context ids of the inputs shall be identical.",
                ));
            }
        }
        Ok(View { program, texture })
    }
}

pub struct TwoNeedleGeometry {
    pub uah: (f32, f32, f32),
    pub needle_pos: (f32, f32, f32),
    pub needle_rot: f32,
    pub needle_length: f32,
}

impl UpdateUniforms for TwoNeedleGeometry {
    fn update_uniforms(&self, program: &Program) -> Result<(), String> {
        program.set_uniform3f("uah", self.uah.0, self.uah.1, self.uah.2)?;
        program.set_uniform3f(
            "npos",
            self.needle_pos.0,
            self.needle_pos.1,
            self.needle_pos.2,
        )?;
        program.set_uniform1f("theta", self.needle_rot)?;
        program.set_uniform1f("L", self.needle_length)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CTPane {
    pub texture: Rc<RefCell<LoadedTexture>>,
    pub z_loc: Rc<RefCell<LoadedTexture>>,
    pub window: f32,
    pub level: f32,
    pub spacing: (f32, f32, f32),
    pub size: (f32, f32, f32),
}

impl BindTextureUniforms for CTPane {
    fn bind_texture_uniforms(&self, program: &Program) -> Result<(), String> {
        program.bind_texture_uniform(&self.texture.borrow(), "sampler0")?;
        program.bind_texture_uniform(&self.z_loc.borrow(), "sampler3")?;
        Ok(())
    }
}

impl UpdateUniforms for CTPane {
    fn update_uniforms(&self, program: &Program) -> Result<(), String> {
        program.set_uniform1f("window", self.window)?;
        program.set_uniform1f("level", self.level)?;
        program.set_uniform3f("spacing0", self.spacing.0, self.spacing.1, self.spacing.2)?;
        program.set_uniform3f("size0", self.size.0, self.size.1, self.size.2)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DosePane {
    pub texture: Rc<RefCell<LoadedTexture>>,
    pub z_loc: Rc<RefCell<LoadedTexture>>,
    pub lut: Rc<RefCell<LoadedTexture>>,
    pub window: f32,
    pub level: f32,
    pub spacing: (f32, f32, f32),
    pub size: (f32, f32, f32),
    pub minmax: (f32, f32),
}

impl BindTextureUniforms for DosePane {
    fn bind_texture_uniforms(&self, program: &Program) -> Result<(), String> {
        program.bind_texture_uniform(&self.texture.borrow(), "sampler1");
        program.bind_texture_uniform(&self.lut.borrow(), "sampler2");
        program.bind_texture_uniform(&self.z_loc.borrow(), "sampler4");
        Ok(())
    }
}

impl UpdateUniforms for DosePane {
    fn update_uniforms(&self, program: &Program) -> Result<(), String> {
        program.set_uniform1f("window1", self.window)?;
        program.set_uniform1f("level1", self.level)?;
        program.set_uniform3f("spacing1", self.spacing.0, self.spacing.1, self.spacing.2)?;
        program.set_uniform3f("size1", self.size.0, self.size.1, self.size.2)?;

        program.set_uniform1f("dl", self.minmax.0)?;
        program.set_uniform1f("dh", self.minmax.1)?;
        // info!("dl: {}, dh: {}", self.minmax.0, self.minmax.1);
        Ok(())
    }
}

pub struct TransverseView {
    pub context: GLContext,
    pub program: Program,
    pub scale: f32,
    pub z: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub size: (i32, i32, i32, i32),
    pub blend: f32,
    pub needles: Rc<RefCell<TwoNeedleGeometry>>,
    pub num_of_indices: i32,
    pub minmax: (f32, f32),

    pub ct: CTPane,
    pub dose: Option<DosePane>,
}

impl TransverseView {
    pub fn bind_texture_uniforms(&self) {
        // self.program.use_program();
        self.ct.bind_texture_uniforms(&self.program);
        if let Some(ref dose) = self.dose {
            dose.bind_texture_uniforms(&self.program);
        }
    }

    pub fn set_window(&mut self, window: f32) -> Result<(), String> {
        self.program.set_uniform1f("window", window as f32)
    }

    pub fn update_uniforms(&mut self) -> Result<(), String> {
        self.program.set_uniform1f("st", self.scale)?;
        self.program.set_uniform1f("slt", self.z);
        self.program.set_uniform1f("ptx", self.pan_x)?;
        self.program.set_uniform1f("pty", self.pan_y)?;

        self.program.set_uniform1f("k", self.blend);

        self.needles.borrow().update_uniforms(&self.program)?;
        self.ct.update_uniforms(&self.program)?;
        if let Some(ref dose) = self.dose {
            dose.update_uniforms(&self.program)?;
        }

        Ok(())
    }
}

impl Rendable for TransverseView {
    fn render(&mut self) {
        let gl = self.context.clone();

        {
            let (x0, y0, width, height) = self.size;
            self.context.viewport(x0, y0, width, height);
        }

        self.program.use_program();
        self.update_uniforms();
        self.bind_texture_uniforms();
        //gl.viewport(0, 0, self.size, self.size);
        gl.draw_elements_with_i32(GL2::TRIANGLES, self.num_of_indices, GL2::UNSIGNED_SHORT, 0);
    }
}

pub struct SagittalView {
    pub context: GLContext,
    pub program: Program,
    pub scale: f32,
    pub x: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub size: (i32, i32, i32, i32),
    pub blend: f32,
    pub needles: Rc<RefCell<TwoNeedleGeometry>>,
    pub num_of_indices: i32,
    pub ct: CTPane,
    pub dose: Option<DosePane>,
}

impl SagittalView {
    pub fn bind_texture_uniforms(&self) {
        self.program.use_program();
        self.ct.bind_texture_uniforms(&self.program);
        if let Some(ref dose) = self.dose {
            dose.bind_texture_uniforms(&self.program);
        }
    }

    pub fn update_uniforms(&mut self) -> Result<(), String> {
        // set window/level
        self.program.use_program();

        self.program.set_uniform1f("ss", self.scale)?;
        self.program.set_uniform1f("sls", self.x);
        self.program.set_uniform1f("psx", self.pan_x)?;
        self.program.set_uniform1f("psy", self.pan_y)?;

        self.program.set_uniform1f("k", self.blend);

        self.needles.borrow_mut().update_uniforms(&mut self.program);
        self.ct.update_uniforms(&self.program)?;
        if let Some(ref dose) = self.dose {
            dose.update_uniforms(&self.program)?;
        }

        Ok(())
    }
}

impl Rendable for SagittalView {
    fn render(&mut self) {
        {
            let (x0, y0, width, height) = self.size;
            self.context.viewport(x0, y0, width, height);
        }

        self.program.use_program();
        self.update_uniforms();
        self.bind_texture_uniforms();
        //gl.viewport(0, 0, self.size, self.size);
        self.context.draw_elements_with_i32(
            GL2::TRIANGLES,
            self.num_of_indices,
            GL2::UNSIGNED_SHORT,
            0,
        );
    }
}
pub struct CoronalView {
    pub context: GLContext,
    pub program: Program,
    pub scale: f32,
    pub y: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub size: (i32, i32, i32, i32),
    pub blend: f32,
    pub needles: Rc<RefCell<TwoNeedleGeometry>>,
    pub num_of_indices: i32,
    pub ct: CTPane,
    pub dose: Option<DosePane>,
}

impl CoronalView {
    pub fn bind_texture_uniforms(&self) {
        self.ct.bind_texture_uniforms(&self.program);
        if let Some(ref dose) = self.dose {
            dose.bind_texture_uniforms(&self.program);
        }
    }

    pub fn update_uniforms(&mut self) -> Result<(), String> {
        self.program.set_uniform1f("sc", self.scale)?;
        self.program.set_uniform1f("slc", self.y);
        self.program.set_uniform1f("pcx", self.pan_x)?;
        self.program.set_uniform1f("pcy", self.pan_y)?;
        self.program.set_uniform1f("k", self.blend);

        self.needles.borrow_mut().update_uniforms(&mut self.program);
        self.ct.update_uniforms(&self.program)?;
        if let Some(ref dose) = self.dose {
            dose.update_uniforms(&self.program)?;
        }

        Ok(())
    }
}

impl Rendable for CoronalView {
    fn render(&mut self) {
        {
            let (x0, y0, width, height) = self.size;
            self.context.viewport(x0, y0, width, height);
        }

        self.program.use_program();
        self.update_uniforms();
        self.bind_texture_uniforms();
        //gl.viewport(0, 0, self.size, self.size);
        self.context.draw_elements_with_i32(
            GL2::TRIANGLES,
            self.num_of_indices,
            GL2::UNSIGNED_SHORT,
            0,
        );
    }
}

pub trait Layout<T> {
    fn layout(&self, ty: &T) -> (i32, i32, i32, i32);
}

pub enum CanvasView {
    Transverse,
    Sagittal,
    Coronal,
    ThreeD,
}

use CanvasView::*;

pub struct LayoutOneLargeThreeSmall {
    width: i32,
    height: i32,
    maximized: CanvasView,
}

impl LayoutOneLargeThreeSmall {
    pub fn new(width: i32, height: i32, maximized: CanvasView) -> LayoutOneLargeThreeSmall {
        LayoutOneLargeThreeSmall { width, height, maximized }
    }

    pub fn set_maximized(&mut self, maximized: CanvasView) {
        self.maximized = maximized;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    fn large(&self) -> (i32, i32, i32, i32) {
        (0, 0, self.height, self.height)
    }

    fn small0(&self) -> (i32, i32, i32, i32) {
        let side_length = (self.height - 2) / 3;
        let x0 = self.height + 1;
        let y0 = side_length * 2 + 2;
        (x0, y0, side_length, side_length)
    }

    fn small1(&self) -> (i32, i32, i32, i32) {
        let side_length = (self.height - 2) / 3;
        let x0 = self.height + 1;
        let y0 = side_length + 1;
        (x0, y0, side_length, side_length)
    }

    fn small2(&self) -> (i32, i32, i32, i32) {
        let side_length = (self.height - 2) / 3;
        let x0 = self.height + 1;
        let y0 = 0;
        (x0, y0, side_length, side_length)
    }
}

impl Layout<CanvasView> for LayoutOneLargeThreeSmall {
    fn layout(&self, ty: &CanvasView) -> (i32, i32, i32, i32) {
        match ty {
            Transverse => match self.maximized {
                Transverse => self.large(),
                _ => self.small0(),
            },
            Sagittal => match self.maximized {
                Transverse => self.small0(),
                Sagittal => self.large(),
                _ => self.small1(),
            },
            Coronal => match self.maximized {
                Coronal => self.large(),
                ThreeD => self.small2(),
                _ => self.small1(),
            },
            ThreeD => match self.maximized {
                ThreeD => self.large(),
                _ => self.small2(),
            }
        }
    }
}

