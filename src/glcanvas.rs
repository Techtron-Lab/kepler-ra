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


#![allow(unused)]
use std::cell::RefCell;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::ops::DerefMut;
use std::rc::Rc;

use do_notation::m;
use js_sys::{ArrayBuffer, Float32Array, Int16Array, Uint16Array, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::{File, FileReader};
use web_sys::{
    Performance,
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader,
    WebGlTexture, //WebGlUniformLocation,
};

// use crate::colormap::cmocean::LUT;
use crate::colormap::jet::LUT;
use crate::modal::{VolumeDataType, VolumeInfo};
use crate::shader_sources::*;
use crate::techtron::prelude::*;
use crate::utils::*;
use crate::view::*;

use log::{info, warn, Level};

const PIXEL_VAL_TO_POSITIVE: u16 = 1500;

macro_rules! set_view_param {
    ($i: expr, $w: ident, $v: expr) => {
        $i.as_mut().map(|x| x.$w = $v);
    };
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_object(v: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_int(v: i32);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ViewType {
    TRANSVERSE = 0,
    SAGITTAL = 1,
    CORONAL = 2,
}

struct ZLocations {
    loc: Vec<f32>,
}

impl ZLocations {
    fn new(gl: WebGl2RenderingContext, id: u32, v: &[f32]) -> Self {
        log("start ZLocations::new");
        type GL2 = WebGl2RenderingContext;

        let mut loc = v.to_vec();
        loc.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ZLocations { loc: loc.clone() }
    }
}

impl GenTexture<Texture3DRGBA16> for ZLocations {
    // type Target = Texture3DRGBA16;
    fn gen_texture3d(&self) -> Texture3DRGBA16 {
        let len = self.loc.len();
        let min = self.loc[0];
        let max = self.loc[len - 1];
        let k = 65535.0;

        let norm_loc: Vec<f32> = self.loc.iter().map(|x| (x - min) / (max - min)).collect();
        let norm_loc_packed: Vec<u16> = norm_loc.iter().map(|x| (x * k) as u16).collect();

        Texture3DRGBA16::new(len as i32, 1, 1, Rc::new(norm_loc_packed))
    }
}

struct GlVolume {
    volume_info: VolumeInfo,
    // context: WebGl2RenderingContext,
    data: Vec<i16>,
}

impl GlVolume {
    fn from_array_buffer(
        // context: WebGl2RenderingContext,
        buffer: ArrayBuffer,
        info: VolumeInfo,
        id: u32,
    ) -> Self {
        // let gl = &context;
        // let texture = gl.create_texture().expect("Failed to create texture.");

        let p = web_sys::window()
            .and_then(|w| w.performance())
            .expect("cannot get the performance object.");
        let t0 = p.now();
        let src = Int16Array::new_with_byte_offset(buffer.as_ref(), 0);
        let mut data: Vec<i16> = src.to_vec();
        for ele in &mut data {
            *ele += PIXEL_VAL_TO_POSITIVE as i16;
        }
        let t1 = p.now();
        log(format!("process dicom in WASM took {} msecs", t1 - t0).as_str());

        GlVolume {
            volume_info: info.clone(),
            // context: context,
            data,
        }
    }
}

impl GenTexture<Texture3DRGBA16> for GlVolume {
    // type Target = Texture3DRGBA16;
    fn gen_texture3d(&self) -> Texture3DRGBA16 {
        let mut data: Vec<u16> = Vec::new();
        for d in &self.data {
            data.push(*d as u16);
        }

        log(&format!("data length: {}", self.data.len()));
        log(&format!("data2 length: {}", data.len()));
        let info = &self.volume_info;
        Texture3DRGBA16::new(
            info.get_width(),
            info.get_height(),
            info.get_depth(),
            Rc::new(data),
        )
    }
}

#[derive(Debug, Clone)]
struct EFVolume {
    dim: (i32, i32, i32),
    spacing: (f32, f32, f32),
    data: Vec<f32>,
}

impl EFVolume {
    pub fn from_array_buffer(
        buf: ArrayBuffer,
        dim: (i32, i32, i32),
        spacing: (f32, f32, f32),
    ) -> EFVolume {
        let f32array = Float32Array::new_with_byte_offset(buf.as_ref(), 0);
        EFVolume {
            dim,
            spacing,
            data: f32array.to_vec(),
        }
    }

    pub fn minmax(&self) -> (f32, f32) {
        self.data.iter().fold((0., 0.), |minmax, x| {
            if x < &minmax.0 {
                (*x, minmax.1)
            } else if x > &minmax.1 {
                (minmax.0, *x)
            } else {
                minmax
            }
        })
    }
}

impl GenTexture<Texture3DRGBA16> for EFVolume {
    fn gen_texture3d(&self) -> Texture3DRGBA16 {
        let minmax = self.minmax();
        let min = minmax.0;
        let max = minmax.1;

        let k = 65535.0;

        let norm: Vec<f32> = self.data.iter().map(|x| (x - min) / (max - min)).collect();
        let norm_packed: Vec<u16> = norm.iter().map(|x| (x * k) as u16).collect();

        info!("norm_packed: {:?}", norm_packed);
        info!("dim: {} {} {}", self.dim.0, self.dim.1, self.dim.2);
        Texture3DRGBA16::new(self.dim.0, self.dim.1, self.dim.2, Rc::new(norm_packed))
    }
}

#[wasm_bindgen]
pub struct GlCanvas {
    context: WebGl2RenderingContext,
    primary_volume: Option<GlVolume>,
    secondary_volume: Option<EFVolume>,
    lut: Rc<RefCell<LoadedTexture>>,
    canvas_dim: (i32, i32),
    primary_slice_locations: Option<ZLocations>,
    secondary_slice_locations: Option<ZLocations>,
    primary_loc_tex: Option<Rc<RefCell<LoadedTexture>>>,
    secondary_loc_tex: Option<Rc<RefCell<LoadedTexture>>>,
    trans_view: Option<TransverseView>,
    sagi_view: Option<SagittalView>,
    coronal_view: Option<CoronalView>,
    geometry: Option<Geometry>,
    rendables: Vec<Box<dyn Rendable>>,
    layout_manager: LayoutOneLargeThreeSmall,
}

#[wasm_bindgen]
impl GlCanvas {
    pub fn new(canvas_id: &str, width: i32, height: i32, win: f32, level: f32) -> GlCanvas {
        set_panic_hook();
        init_log();
        let context = get_context_by_canvas_id(canvas_id).unwrap();
        info!("GlCanvas::new");
        // let program = load_shaders(context.clone(), VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE_NOIMAGE).unwrap();
        // let program = Rc::new(RefCell::new(
        //     load_shaders(&context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap(),
        // ));
        let lut_data = Rc::new(crate::colormap::jet::LUT.to_vec());
        let lut = Rc::new(RefCell::new(
            Texture3DRGB8::new(256, 1, 1, lut_data).load_texture(&GLContext::new(context.clone())),
        ));
        let mut glcanvas = GlCanvas {
            context: context.clone(),
            // program: program.clone(),
            primary_volume: None, //Some(GlVolume::new(context.clone(), 0, 65535)),
            secondary_volume: None, //Some(GlVolume::new(context.clone(), 1, 65535)),
            lut,
            canvas_dim: (width, height),
            primary_slice_locations: None,
            secondary_slice_locations: None,
            primary_loc_tex: None,
            secondary_loc_tex: None,
            trans_view: None,
            sagi_view: None,
            coronal_view: None,
            rendables: Vec::new(),
            geometry: None,
            layout_manager: LayoutOneLargeThreeSmall::new(width, height, CanvasView::Transverse),
        };
        return glcanvas;
    }

    fn set_scale(
        &mut self,
        transverse: Option<f32>,
        sagittal: Option<f32>,
        coronal: Option<f32>,
    ) -> Result<(), JsValue> {
        if let Some(v) = transverse {
            self.set_scale_transverse(v)?;
        }
        if let Some(v) = sagittal {
            self.set_scale_sagittal(v)?;
        }
        if let Some(v) = coronal {
            self.set_scale_coronal(v)?;
        }
        Ok(())
    }

    pub fn set_canvas_dim(&mut self, w: i32, h: i32) -> Result<(), JsValue> {
        self.canvas_dim = (w, h);
        Ok(())
    }

    fn set_default_uniform_values(&mut self) -> Result<(), JsValue> {
        self.set_scale(Some(1.0), Some(1.0), Some(1.0))?;
        self.set_pan_transverse_x(0.0)?;
        self.set_pan_transverse_y(0.0)?;
        self.set_pan_sagittal_x(0.0)?;
        self.set_pan_sagittal_y(0.0)?;
        self.set_pan_coronal_x(0.0)?;
        self.set_pan_coronal_y(0.0)?;
        self.set_slice_transverse(0.0)?;
        self.set_slice_sagittal(0.0)?;
        self.set_slice_coronal(0.0)?;
        self.set_blend(0.0)?;
        Ok(())
    }

    // pub fn set_shift(&mut self, s0: f32, s1: f32, s2: f32) -> Result<(), JsValue> {
    //     log(format!("shift: {} {} {}", s0, s1, s2).as_str());
    //     self.set_uniform3f("shift", s0, s1, s2)
    // }

    // pub fn get_shift(&self) -> Result<Float32Array, JsValue> {
    //     self.get_uniform3f("shift")
    // }

    pub fn set_window(&mut self, window: f32) -> Result<(), JsValue> {
        self.set_primary_window(window);
        self.set_secondary_window(window);
        Ok(())
    }

    // pub fn get_window(&self) -> Result<f32, JsValue> {
    //     self.get_uniform1f("window")
    // }

    pub fn set_level(&mut self, level: f32) -> Result<(), JsValue> {
        self.set_primary_level(level);
        self.set_secondary_level(level);
        Ok(())
    }

    // pub fn get_level(&self) -> Result<f32, JsValue> {
    //     self.get_uniform1f("level")
    // }

    pub fn set_primary_window(&mut self, window: f32) -> Result<(), JsValue> {
        self.trans_view.as_mut().map(|x| x.ct.window = window);
        // self.trans_view.as_mut().map(|x| x.primary_window = window);
        self.sagi_view.as_mut().map(|x| x.ct.window = window);
        self.coronal_view.as_mut().map(|x| x.ct.window = window);
        Ok(())
    }

    pub fn get_primary_window(&self) -> f32 {
        self.trans_view.as_ref().map(|v| v.ct.window).unwrap()
        // self.trans_view.as_ref().map(|v| v.primary_window).unwrap()
    }

    pub fn set_primary_level(&mut self, level: f32) -> Result<(), JsValue> {
        let lev = level + PIXEL_VAL_TO_POSITIVE as f32;
        self.trans_view.as_mut().map(|x| x.ct.level = lev);
        // self.trans_view.as_mut().map(|x| x.primary_level = lev);
        self.sagi_view.as_mut().map(|x| x.ct.level = lev);
        self.coronal_view.as_mut().map(|x| x.ct.level = lev);
        Ok(())
    }

    pub fn get_primary_level(&self) -> f32 {
        self.trans_view.as_ref().map(|v| v.ct.level).unwrap()
        // self.trans_view.as_ref().map(|v| v.primary_level).unwrap()
    }

    pub fn set_secondary_window(&mut self, window: f32) -> Result<(), JsValue> {
        self.trans_view
            .as_mut()
            // .map(|x| x.secondary_window = window);
            .map(|x| x.dose.as_mut().map(|d| d.window = window));
        self.sagi_view
            .as_mut()
            .map(|x| x.dose.as_mut().map(|d| d.window = window));
        self.coronal_view
            .as_mut()
            .map(|x| x.dose.as_mut().map(|d| d.window = window));
        Ok(())
    }

    pub fn get_secondary_window(&self) -> Result<f32, String> {
        if let Some(ref view) = self.trans_view {
            if let Some(ref dose) = view.dose {
                return Ok(dose.window);
            }
        }
        Err("Dose hasn't been initialized".to_string())
    }

    pub fn set_secondary_level(&mut self, level: f32) -> Result<(), JsValue> {
        //let lev = level + PIXEL_VAL_TO_POSITIVE as f32;
        let lev = level;
        self.trans_view
            .as_mut()
            .map(|x| x.dose.as_mut().map(|d| d.level = lev));
        self.sagi_view
            .as_mut()
            .map(|x| x.dose.as_mut().map(|d| d.level = lev));
        self.coronal_view
            .as_mut()
            .map(|x| x.dose.as_mut().map(|d| d.level = lev));
        Ok(())
    }

    pub fn get_secondary_level(&self) -> Result<f32, String> {
        // self.trans_view.as_ref().map(|v| v.dose.as_ref().map(|d| d.level).unwrap()).unwrap()
        if let Some(ref view) = self.trans_view {
            if let Some(ref dose) = view.dose {
                return Ok(dose.level);
            }
        }
        Err("Dose hasn't been initialized".to_string())
    }
    pub fn set_scale_transverse(&mut self, scale: f32) -> Result<(), JsValue> {
        self.trans_view.as_mut().map(|x| x.scale = scale);
        Ok(())
    }

    pub fn get_scale_transverse(&self) -> Result<f32, JsValue> {
        self.trans_view
            .as_ref()
            .map(|v| v.scale)
            .ok_or("data hasn't been initialized".into())
    }
    pub fn set_scale_sagittal(&mut self, scale: f32) -> Result<(), JsValue> {
        // self.sagi_view.as_mut().map(|x| x.scale = scale);
        set_view_param!(self.sagi_view, scale, scale);
        Ok(())
    }

    pub fn get_scale_sagittal(&self) -> Result<f32, JsValue> {
        self.sagi_view
            .as_ref()
            .map(|v| v.scale)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_scale_coronal(&mut self, scale: f32) -> Result<(), JsValue> {
        self.coronal_view.as_mut().map(|x| x.scale = scale);
        Ok(())
    }

    pub fn get_scale_coronal(&self) -> Result<f32, JsValue> {
        self.coronal_view
            .as_ref()
            .map(|v| v.scale)
            .ok_or("data hasn't been initialized".into())
    }

    // fn set_type(&mut self, view_type: ViewType) -> Result<(), JsValue> {
    //     self.set_uniform1i("type", view_type as i32)

    // }

    // fn show_secondary(&mut self, show: bool) -> Result<(), JsValue> {
    //     self.set_uniform1i("s", if show { 1 } else { 0 })
    // }

    pub fn set_pan_transverse(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        self.set_pan_transverse_x(x)?;
        self.set_pan_transverse_y(y)
    }

    pub fn set_pan_transverse_x(&mut self, x: f32) -> Result<(), JsValue> {
        self.trans_view.as_mut().map(|v| v.pan_x = x);
        Ok(())
    }

    pub fn get_pan_transverse_x(&self) -> Result<f32, JsValue> {
        self.trans_view
            .as_ref()
            .map(|v| v.pan_x)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_pan_transverse_y(&mut self, y: f32) -> Result<(), JsValue> {
        self.trans_view.as_mut().map(|v| v.pan_y = y);
        Ok(())
    }

    pub fn get_pan_transverse_y(&self) -> Result<f32, JsValue> {
        self.trans_view
            .as_ref()
            .map(|v| v.pan_y)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_pan_sagittal_x(&mut self, x: f32) -> Result<(), JsValue> {
        self.sagi_view.as_mut().map(|v| v.pan_x = x);
        Ok(())
    }

    pub fn get_pan_sagittal_x(&self) -> Result<f32, JsValue> {
        self.sagi_view
            .as_ref()
            .map(|v| v.pan_x)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_pan_sagittal_y(&mut self, y: f32) -> Result<(), JsValue> {
        self.sagi_view.as_mut().map(|v| v.pan_y = y);
        Ok(())
    }

    pub fn get_pan_sagittal_y(&self) -> Result<f32, JsValue> {
        self.sagi_view
            .as_ref()
            .map(|v| v.pan_y)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_pan_coronal_x(&mut self, x: f32) -> Result<(), JsValue> {
        self.coronal_view.as_mut().map(|v| v.pan_x = x);
        Ok(())
    }

    pub fn get_pan_coronal_x(&self) -> Result<f32, JsValue> {
        self.coronal_view
            .as_ref()
            .map(|v| v.pan_x)
            .ok_or("data hasn't been initialized".into())
    }
    pub fn set_pan_coronal_y(&mut self, z: f32) -> Result<(), JsValue> {
        self.coronal_view.as_mut().map(|v| v.pan_y = z);
        Ok(())
    }

    pub fn get_pan_coronal_y(&self) -> Result<f32, JsValue> {
        self.coronal_view
            .as_ref()
            .map(|v| v.pan_y)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_slice_transverse(&mut self, slice: f32) -> Result<(), JsValue> {
        self.trans_view.as_mut().map(|v| v.z = slice);
        Ok(())
    }

    pub fn get_slice_transverse(&self) -> Result<f32, JsValue> {
        self.trans_view
            .as_ref()
            .map(|v| v.z)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_slice_sagittal(&mut self, slice: f32) -> Result<(), JsValue> {
        self.sagi_view.as_mut().map(|v| v.x = slice);
        Ok(())
    }
    pub fn get_slice_sagittal(&self) -> Result<f32, JsValue> {
        self.sagi_view
            .as_ref()
            .map(|v| v.x)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_slice_coronal(&mut self, slice: f32) -> Result<(), JsValue> {
        self.coronal_view.as_mut().map(|v| v.y = slice);
        Ok(())
    }

    pub fn get_slice_coronal(&self) -> Result<f32, JsValue> {
        self.coronal_view
            .as_ref()
            .map(|v| v.y)
            .ok_or("data hasn't been initialized".into())
    }

    pub fn set_blend(&mut self, k: f32) -> Result<(), JsValue> {
        if k < 0.0 || k > 1.0 {
            return Err(JsValue::from("blend shall be in between 0.0 and 1.0."));
        }
        self.trans_view.as_mut().map(|v| v.blend = k);
        self.sagi_view.as_mut().map(|v| v.blend = k);
        self.coronal_view.as_mut().map(|v| v.blend = k);
        Ok(())
    }

    // pub fn set_secondary_lut(&mut self, lut: bool) -> Result<(), JsValue> {
    //     self.set_uniform1i("lut", if lut { 1 } else { 0 })
    // }

    pub fn get_transverse_coord(&self, x: f32, y: f32) -> Result<Box<[f32]>, JsValue> {
        let scale = &self.get_scale_transverse()?;
        let pan_x = &self.get_pan_transverse_x()?;
        let pan_y = &self.get_pan_transverse_y()?;
        let x1 = (x - pan_x) / scale * 250.0;
        let y1 = (y - pan_y) / scale * 250.0;
        let v = vec![x1, y1];
        Ok(v.into_boxed_slice())
    }

    pub fn get_sagittal_coord(&self, x: f32, y: f32) -> Result<Box<[f32]>, JsValue> {
        let scale = &self.get_scale_sagittal()?;
        let pan_x = &self.get_pan_sagittal_x()?;
        let pan_y = &self.get_pan_sagittal_y()?;
        let x1 = (x - pan_x) / scale * 250.0;
        let y1 = (y - pan_y) / scale * 250.0;
        let v = vec![x1, y1];
        Ok(v.into_boxed_slice())
    }

    pub fn get_coronal_coord(&self, x: f32, y: f32) -> Result<Box<[f32]>, JsValue> {
        let scale = &self.get_scale_coronal()?;
        let pan_x = &self.get_pan_coronal_x()?;
        let pan_y = &self.get_pan_coronal_y()?;
        let x1 = (x - pan_x) / scale * 250.0;
        let y1 = (y - pan_y) / scale * 250.0;
        let v = vec![x1, y1];
        Ok(v.into_boxed_slice())
    }

    pub fn get_transverse_value_in_primary(&self, x: f32, y: f32) -> Result<i16, JsValue> {
        let scale = &self.get_scale_transverse()?;
        let pan_x = &self.get_pan_transverse_x()?;
        let pan_y = &self.get_pan_transverse_y()?;
        let primary = self.primary_volume.as_ref().ok_or("error: primary")?;
        let (width, height, depth) = primary.volume_info.get_dimension();
        let (sx, sy, sz) = primary.volume_info.get_spacing().ok_or("error")?;
        let z_locations = self
            .primary_slice_locations
            .as_ref()
            .ok_or("cannot retrieve slice locations")?;
        let zloc = &z_locations.loc;
        let len = zloc.len();
        let min = zloc[0];
        let max = zloc[len - 1];
        let data = &primary.data;
        let x1 = x; //(x - pan_x) / scale;
        let y1 = y; //(y - pan_y) / scale;
        let z1 = &self.get_slice_transverse()?;

        // do the calculation
        // let nx: usize = (x1 / sx * 250.0 + (width as f32) / 2.0).round() as usize;
        // let ny: usize = (-y1 / sy * 250.0 + (height as f32) / 2.0).round() as usize;
        let nx: usize = (x1 / sx + (width as f32) / 2.0).round() as usize;
        let ny: usize = (-y1 / sy + (height as f32) / 2.0).round() as usize;

        // let norm_loc: Vec<f32> = loc.iter().map(|x| (x - min) / (max - min)).collect();
        let k = (max - min) / 500.0;
        let nz: usize = zloc
            .iter()
            .map(|x| k * ((x - min) / (max - min) * 2.0 - 1.0))
            .map(|x| (x - z1).abs())
            .enumerate()
            .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(Ordering::Equal))
            .ok_or("cannot find z slice")?
            .0;
        log_object(unsafe { &js_sys::Float32Array::view(&zloc) });
        log(format!(
            "x:{}, x1:{}, nx: {}, sx: {}, width: {}",
            x, x1, nx, sx, width
        )
        .as_str());
        log(format!(
            "y:{}, y1:{}, ny: {}, sy: {}, width: {}",
            y, y1, ny, sy, width
        )
        .as_str());
        log(format!(
            "z:{}, z1:{}, nz: {}, sz: {}, width: {}",
            z1 * 250.0,
            z1,
            nz,
            sz,
            width
        )
        .as_str());
        let index = nz * (width * height) as usize + ny * width as usize + nx;
        let value = data[index] - PIXEL_VAL_TO_POSITIVE as i16;
        Ok(value)
    }

    pub fn get_normalized_slice_loc(&self) -> Result<Box<[f32]>, JsValue> {
        //let primary = self.primary.as_ref().ok_or("error: primary")?;
        //let zloc = primary.volume_info.get_slice_locations().ok_or("cannot retrieve slice locations")?;
        let z_locations = self
            .primary_slice_locations
            .as_ref()
            .ok_or("cannot retrieve slice locations")?;
        let zloc = &z_locations.loc;
        let len = zloc.len();
        let min = zloc[0];
        let max = zloc[len - 1];
        let k = (max - min) / 500.0;
        let norm_loc: Vec<f32> = zloc
            .iter()
            .map(|x| k * ((x - min) / (max - min) * 2.0 - 1.0))
            .collect();
        Ok(norm_loc.into_boxed_slice())
    }

    fn set_primary_spacing(&mut self, spacing: (f32, f32, f32)) -> Result<(), JsValue> {
        log(format!("Set spacing of the primary: {:?}", spacing).as_str());
        // self.set_uniform3f("spacing0", spacing.0, spacing.1, spacing.2)
        self.trans_view
            .as_mut()
            .map(|v| v.ct.spacing = spacing)
            // .map(|v| v.spacing0 = spacing)
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.sagi_view
            .as_mut()
            .map(|v| v.ct.spacing = spacing)
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.coronal_view
            .as_mut()
            .map(|v| v.ct.spacing = spacing)
            .ok_or("data hasn't been initialized".into())
    }

    fn set_secondary_spacing(&mut self, spacing: (f32, f32, f32)) -> Result<(), JsValue> {
        log(format!("Set spacing of the secondary: {:?}", spacing).as_str());
        // self.set_uniform3f("spacing1", spacing.0, spacing.1, spacing.2)
        self.trans_view
            .as_mut()
            .map(|v| v.dose.as_mut().map(|d| d.spacing = spacing))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.sagi_view
            .as_mut()
            .map(|v| v.dose.as_mut().map(|d| d.spacing = spacing))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.coronal_view
            .as_mut()
            .map(|v| v.dose.as_mut().map(|d| d.spacing = spacing))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        Ok(())
    }

    // fn set_primary_dim(&mut self, d0: f32, d1: f32, d2: f32) -> Result<(), JsValue> {
    //     log(format!("Set dim of the primary: ({}, {}, {})", d0, d1, d2).as_str());
    //     self.set_uniform3f("dim0", d0, d1, d2)
    // }

    // fn set_secondary_dim(&mut self, d0: f32, d1: f32, d2: f32) -> Result<(), JsValue> {
    //     log(format!("Set dim of the secondary: ({}, {}, {})", d0, d1, d2).as_str());
    //     self.set_uniform3f("dim1", d0, d1, d2)
    // }

    fn set_primary_size(&mut self, s0: f32, s1: f32, s2: f32) -> Result<(), JsValue> {
        // self.set_uniform3f("size0", s0, s1, s2)
        self.trans_view
            .as_mut()
            .map(|v| v.ct.size = (s0, s1, s2))
            // .map(|v| v.size0 = (s0, s1, s2))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.sagi_view
            .as_mut()
            .map(|v| v.ct.size = (s0, s1, s2))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.coronal_view
            .as_mut()
            .map(|v| v.ct.size = (s0, s1, s2))
            .ok_or("data hasn't been initialized".into())
    }

    fn set_secondary_size(&mut self, s0: f32, s1: f32, s2: f32) -> Result<(), JsValue> {
        // self.set_uniform3f("size1", s0, s1, s2)
        self.trans_view
            .as_mut()
            .map(|v| v.dose.as_mut().map(|d| d.size = (s0, s1, s2)))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.sagi_view
            .as_mut()
            .map(|v| v.dose.as_mut().map(|d| d.size = (s0, s1, s2)))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;
        self.coronal_view
            .as_mut()
            // .map(|v| v.size1 = (s0, s1, s2))
            .map(|v| v.dose.as_mut().map(|d| d.size = (s0, s1, s2)))
            .ok_or::<JsValue>("data hasn't been initialized".into())?;

        Ok(())
    }

    pub fn set_uah(&mut self, u0: f32, a: f32, h: f32) -> Result<(), JsValue> {
        log(format!("u0: {}, a: {}, h: {}", u0, a, h).as_str());
        self.trans_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().uah = (u0, a, h));
        self.sagi_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().uah = (u0, a, h));
        self.coronal_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().uah = (u0, a, h));
        Ok(())
    }

    pub fn set_needle_pos(&mut self, x: f32, y: f32, z: f32) -> Result<(), JsValue> {
        log(format!("x: {}, y: {}, z: {}", x, y, z).as_str());
        self.trans_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_pos = (x, y, z));
        self.sagi_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_pos = (x, y, z));
        self.coronal_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_pos = (x, y, z));
        Ok(())
    }

    pub fn set_needle_rot(&mut self, theta: f32) -> Result<(), JsValue> {
        log(format!("theta: {}", theta).as_str());
        self.trans_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_rot = theta);
        self.sagi_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_rot = theta);
        self.coronal_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_rot = theta);
        Ok(())
    }
    pub fn set_needle_length(&mut self, len: f32) -> Result<(), JsValue> {
        log(format!("L: {}", len).as_str());
        self.trans_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_length = len);
        self.sagi_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_length = len);
        self.coronal_view
            .as_mut()
            .map(|v| v.needles.borrow_mut().needle_length = len);
        Ok(())
    }

    pub fn update_canvas_dim(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
        self.trans_view.as_mut().map(|v| {
            self.layout_manager.set_width(width);
            self.layout_manager.set_height(height);
            // v.size = self.layout_manager.layout(&CanvasView::Transverse);
        });
        self.sagi_view.as_mut().map(|v| {
            self.layout_manager.set_width(width);
            self.layout_manager.set_height(height);
            // v.size = self.layout_manager.layout(&CanvasView::Sagittal);
        });
        self.coronal_view.as_mut().map(|v| {
            self.layout_manager.set_width(width);
            self.layout_manager.set_height(height);
            // v.size = self.layout_manager.layout(&CanvasView::Coronal);
        });
        Ok(())
    }

    pub fn maximize(&mut self, t: &str) -> Result<(), JsValue> {
        match t {
            "T" => {
                self.layout_manager.set_maximized(CanvasView::Transverse);
                // self.update_layout();
                return Ok(());
            }
            "S" => {
                self.layout_manager.set_maximized(CanvasView::Sagittal);
                // self.update_layout();
                return Ok(());
            }
            "C" => {
                self.layout_manager.set_maximized(CanvasView::Coronal);
                // self.update_layout();
                return Ok(());
            }
            "3D" => {
                self.layout_manager.set_maximized(CanvasView::ThreeD);
                // self.update_layout();
                return Ok(());
            }
            _ => return Err(JsValue::undefined()),
        }
    }

    fn update_layout(&mut self) {
        self.trans_view
            .as_mut()
            .map(|v| v.size = self.layout_manager.layout(&CanvasView::Transverse));
        self.sagi_view
            .as_mut()
            .map(|v| v.size = self.layout_manager.layout(&CanvasView::Sagittal));
        self.coronal_view
            .as_mut()
            .map(|v| v.size = self.layout_manager.layout(&CanvasView::Coronal));
    }

    pub fn load_primary(
        &mut self,
        buffer: ArrayBuffer,
        w: i32,
        h: i32,
        d: i32,
        spacing_x: f32,
        spacing_y: f32,
        spacing_z: f32,
    ) -> Result<(), JsValue> {
        // self.program = Self::load_shaders(self.context.clone(), VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let spacing = (spacing_x, spacing_y, spacing_z);
        let mut vinfo = VolumeInfo::new(VolumeDataType::Int16, w, h, d);
        vinfo.set_spacing(spacing);
        // if let Some(primary) = self.primary.as_mut() {
        //     primary.load(buffer, vinfo);
        // } else {
        //     self.primary = Some(GlVolume::from_array_buffer(self.context.clone(), buffer, vinfo, 0));
        // }
        self.primary_volume = Some(GlVolume::from_array_buffer(
            // self.context.clone(),
            buffer, vinfo, 0,
        ));
        // self.set_primary_dim(w as f32, h as f32, d as f32)?;
        self.set_primary_spacing(spacing)?;
        self.set_primary_size(
            w as f32 * spacing_x,
            h as f32 * spacing_y,
            d as f32 * spacing_z,
        )?;

        // Make the Z locations which is evenly distributed in this case
        let mut loc: Vec<f32> = vec![0.0];
        for i in 1..d {
            loc.push(spacing_z * i as f32);
        }
        let zloc = ZLocations::new(self.context.clone(), 4, loc.as_slice());
        self.primary_slice_locations = Some(zloc);

        Ok(())
    }

    pub fn load_primary_with_slice_locations(
        &mut self,
        buffer: ArrayBuffer,
        w: i32,
        h: i32,
        spacing_x: f32,
        spacing_y: f32,
        loc: Float32Array,
    ) -> Result<(), JsValue> {
        info!("info: load_primary_with_slice_locations");
        // self.program.borrow().use_program();
        let z = loc.to_vec();
        let d = z.len();
        let size_z: f32 = z[d - 1] - z[0];
        let spacing_z: f32 = size_z / (d - 1) as f32;

        let spacing = (spacing_x, spacing_y, spacing_z);
        let mut vinfo = VolumeInfo::new(VolumeDataType::Int16, w, h, d as i32);
        vinfo.set_spacing(spacing);
        self.primary_volume = Some(GlVolume::from_array_buffer(
            // self.context.clone(),
            buffer, vinfo, 0,
        ));

        let zloc = ZLocations::new(self.context.clone(), 4, z.as_slice());
        log("after ZLocations::new");
        let zloc_tex = zloc
            .gen_texture3d()
            .load_texture(&GLContext::new(self.context.clone()));
        self.primary_loc_tex = Some(Rc::new(RefCell::new(zloc_tex)));
        self.primary_slice_locations = Some(zloc);

        if let Some(volume) = self.primary_volume.as_ref() {
            let texture = Rc::new(RefCell::new(
                volume
                    .gen_texture3d()
                    .load_texture(&GLContext::new(self.context.clone())),
            ));
            let trans_prog = load_shaders(
                &self.context,
                VERTEX_SHADER_SOURCE,
                FRAGMENT_SHADER_SOURCE_TRANS,
            )?;
            let needles = Rc::new(RefCell::new(TwoNeedleGeometry {
                uah: (15000., 0.5, 10.),
                needle_length: 15.,
                needle_rot: 0.,
                needle_pos: (0., 0., 0.),
            }));
            let ct = CTPane {
                texture: texture.clone(),
                z_loc: self.primary_loc_tex.as_ref().unwrap().clone(),
                window: 600.,
                level: 1600.,
                spacing,
                size: (w as f32 * spacing_x, h as f32 * spacing_y, size_z),
            };
            info!("primary spacing: {:?}", &ct.spacing);
            info!("primary size: {:?}", &ct.size);

            let trans_view = TransverseView {
                context: GLContext::new(self.context.clone()),
                program: trans_prog,
                scale: 1.,
                z: 0.,
                pan_x: 0.,
                pan_y: 0.,
                size: self.layout_manager.layout(&CanvasView::Transverse),
                blend: 0.5,
                needles: needles.clone(),
                num_of_indices: 6,
                minmax: (0., 0.),
                ct: ct.clone(),
                dose: None,
            };
            let sagi_prog = load_shaders(
                &self.context,
                VERTEX_SHADER_SOURCE,
                FRAGMENT_SHADER_SOURCE_SAGITTAL,
            )?;

            let sagi_view = SagittalView {
                context: GLContext::new(self.context.clone()),
                program: sagi_prog,
                scale: 1.,
                x: 0.,
                pan_x: 0.,
                pan_y: 0.,
                size: self.layout_manager.layout(&CanvasView::Sagittal),
                blend: 0.5,
                needles: needles.clone(),
                num_of_indices: 6,
                ct: ct.clone(),
                dose: None,
            };

            let coronal_prog = load_shaders(
                &self.context,
                VERTEX_SHADER_SOURCE,
                FRAGMENT_SHADER_SOURCE_CORONAL,
            )?;
            let coronal_view = CoronalView {
                context: GLContext::new(self.context.clone()),
                program: coronal_prog,
                scale: 1.,
                y: 0.,
                pan_x: 0.,
                pan_y: 0.,
                size: self.layout_manager.layout(&CanvasView::Coronal),
                blend: 0.5,
                needles: needles.clone(),
                num_of_indices: 6,
                ct: ct.clone(),
                dose: None,
            };

            self.trans_view = Some(trans_view);
            self.sagi_view = Some(sagi_view);
            self.coronal_view = Some(coronal_view);
        }
        self.set_primary_spacing(spacing)?;
        self.set_primary_size(w as f32 * spacing_x, h as f32 * spacing_y, size_z)?;

        log("finished load_primary_with_slice_locations");
        Ok(())
    }

    pub fn load_secondary(
        &mut self,
        buffer: ArrayBuffer,
        w: i32,
        h: i32,
        d: i32,
        spacing_x: f32,
        spacing_y: f32,
        spacing_z: f32,
    ) -> Result<(), JsValue> {
        info!("info: load_secondary");
        self.trans_view.as_ref().unwrap().program.use_program();
        let spacing = (spacing_x, spacing_y, spacing_z);
        let mut info = VolumeInfo::new(VolumeDataType::Int16, w, h, d);
        info.set_spacing(spacing);
        self.secondary_volume = Some(EFVolume::from_array_buffer(
            buffer,
            (w, h, d),
            (spacing_x, spacing_y, spacing_z),
        ));
        // self.set_secondary_dim(w as f32, h as f32, d as f32)?;
        self.set_secondary_spacing(spacing)?;
        self.set_secondary_size(
            w as f32 * spacing_x,
            h as f32 * spacing_y,
            d as f32 * spacing_z,
        )?;

        // Make the Z locations which is evenly distributed in this case
        let mut loc: Vec<f32> = vec![0.0];
        for i in 1..d {
            loc.push(spacing_z * i as f32);
        }
        let zloc = ZLocations::new(self.context.clone(), 5, loc.as_slice());
        let zloc_tex = zloc
            .gen_texture3d()
            .load_texture(&GLContext::new(self.context.clone()));
        self.secondary_loc_tex = Some(Rc::new(RefCell::new(zloc_tex)));
        self.secondary_slice_locations = Some(zloc);

        // self.trans_view
        //     .as_mut()
        //     .map(|v| v.z_loc2 = Some(self.secondary_loc_tex.as_ref().unwrap().clone()));

        if let Some(volume) = self.secondary_volume.as_ref() {
            let texture = Rc::new(RefCell::new(
                volume
                    .gen_texture3d()
                    .load_texture(&GLContext::new(self.context.clone())),
            ));
            let dose = DosePane {
                texture: texture.clone(),
                z_loc: self.secondary_loc_tex.as_ref().unwrap().clone(),
                lut: self.lut.clone(),
                window: 300.,
                level: 1200.,
                spacing: (spacing_x, spacing_y, spacing_z),
                size: (
                    w as f32 * spacing_x,
                    h as f32 * spacing_y,
                    d as f32 * spacing_z,
                ),
                minmax: volume.minmax(),
            };
            info!("secondary spacing: {:?}", &dose.spacing);
            info!("secondary size: {:?}", &dose.size);
            self.trans_view.as_mut().map(|v| {
                v.dose = Some(dose.clone());
            });

            self.sagi_view.as_mut().map(|v| {
                v.dose = Some(dose.clone());
            });
            self.coronal_view.as_mut().map(|v| {
                v.dose = Some(dose.clone());
            });
        }

        // self.show_secondary(true)?;

        Ok(())
    }

    pub fn load_secondary_with_slice_locations(
        &mut self,
        buffer: ArrayBuffer,
        w: i32,
        h: i32,
        spacing_x: f32,
        spacing_y: f32,
        loc: Float32Array,
    ) -> Result<(), JsValue> {
        // self.load_shaders(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;

        let z = loc.to_vec();
        let d = z.len();
        let size_z: f32 = z[d - 1] - z[0];
        let spacing_z: f32 = size_z / (d - 1) as f32;

        let spacing = (spacing_x, spacing_y, spacing_z);
        let mut vinfo = VolumeInfo::new(VolumeDataType::Int16, w, h, d as i32);
        vinfo.set_spacing(spacing);
        self.secondary_volume = Some(EFVolume::from_array_buffer(
            buffer,
            (w, h, d as i32),
            (spacing_x, spacing_y, spacing_z),
        ));
        self.set_secondary_size(w as f32 * spacing_x, h as f32 * spacing_y, size_z)?;

        let zloc = ZLocations::new(self.context.clone(), 5, z.as_slice());
        self.secondary_slice_locations = Some(zloc);

        Ok(())
    }

    pub fn setup_geometry(&mut self) -> Result<(), JsValue> {
        let context = &self.context;
        type GL2 = WebGl2RenderingContext;

        if self.geometry.is_none() {
            let vertices = vec![
                -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0,
            ];
            let indices = vec![0, 1, 2, 0, 2, 3];
            // self.num_of_indices = indices.len() as i32;

            let glcontext = GLContext::new(self.context.clone());

            let vbuf = VertexBuffer::new(vertices).load_buffer(&glcontext)?;
            let ibuf = IndexBuffer::new(indices).load_buffer(&glcontext)?;

            let geometry = Geometry {
                context: glcontext,
                vbuf,
                ibuf,
            };
            geometry.enable_buffer();
            self.geometry = Some(geometry);
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        let gl = self.context.clone();
        type GL2 = WebGl2RenderingContext;
        gl.clear_color(0.5, 0.5, 0.5, 1.0);
        gl.clear(GL2::COLOR_BUFFER_BIT | GL2::DEPTH_BUFFER_BIT);

        info!("render...");
        self.update_layout();
        self.geometry.as_ref().map(|geo| geo.enable_buffer());
        self.trans_view.as_mut().map(|v| v.render());
        self.sagi_view.as_mut().map(|v| v.render());
        self.coronal_view.as_mut().map(|v| v.render());
        /* info!("renables: {}", self.rendables.len()); */
        /* for view in &mut self.rendables { */
        /*     info!("redering"); */
        /*     view.render(); */
        /* } */
        Ok(())
    }
}

impl Drop for GlCanvas {
    fn drop(&mut self) {
        log("dropping");
    }
}

pub fn get_context_by_canvas_id(canvas_id: &str) -> Result<WebGl2RenderingContext, JsValue> {
    // let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document.get_element_by_id(canvas_id).unwrap();

    let canvas = web_sys::window()
        .ok_or("cannot get window object")?
        .document()
        .ok_or("cannot obtain the document object")?
        .get_element_by_id(canvas_id)
        .ok_or("cannot obtain the canvas.")?;

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl2")?
        .ok_or("cannot obtain webgl2 context")?
        .dyn_into::<WebGl2RenderingContext>()?;

    Ok(context)
}
