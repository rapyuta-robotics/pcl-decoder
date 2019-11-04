extern crate js_sys;
extern crate console_error_panic_hook;

use hsl::HSL;
use scroll::Pread;
use wasm_bindgen::prelude::*;

// usize here represents 32bits for the pointer space, since wasm target is 32bit
const MEMORY_WIDTH: usize = 5_000_000 * 32;

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub fn get_memory_width() -> usize {
    MEMORY_WIDTH
}

#[wasm_bindgen]
pub struct PCLDecoder {
    copy_memory_buffer: [u8; MEMORY_WIDTH],
    position_memory_buffer: [f32; MEMORY_WIDTH / 4],
    color_memory_buffer: [f32; MEMORY_WIDTH / 4],
}

#[wasm_bindgen]
impl PCLDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PCLDecoder {
        console_error_panic_hook::set_once();
        PCLDecoder {
            copy_memory_buffer: [0; MEMORY_WIDTH],
            position_memory_buffer: [0.0; MEMORY_WIDTH / 4],
            color_memory_buffer: [0.0; MEMORY_WIDTH / 4]
        }
    }

    pub fn get_copy_memory_ptr(&self) -> *const u8 {
        self.copy_memory_buffer.as_ptr()
    }
    pub fn get_position_memory_ptr(&self) -> *const f32 {
        self.position_memory_buffer.as_ptr()
    }
    pub fn get_color_memory_ptr(&self) -> *const f32 {
        self.color_memory_buffer.as_ptr()
    }
    pub fn compute(
        &mut self,
        num_points: usize,
        point_step: usize,
        offset_x: usize,
        offset_y: usize,
        offset_z: usize,
        offset_rgb: usize,
        offset_intensity: usize,
        use_intensity_channel: bool,
        use_rainbow: bool,
    ) {
        for i in 0..num_points {
            let stride: usize = i * point_step;
            self.position_memory_buffer[3 * i] = self
                .copy_memory_buffer
                .pread_with::<f32>(stride + offset_x, scroll::LE)
                .unwrap();
            self.position_memory_buffer[3 * i + 1] = self
                .copy_memory_buffer
                .pread_with::<f32>(stride + offset_y, scroll::LE)
                .unwrap();
            self.position_memory_buffer[3 * i + 2] = self
                .copy_memory_buffer
                .pread_with::<f32>(stride + offset_z, scroll::LE)
                .unwrap();

            if offset_rgb != 0 {
                self.color_memory_buffer[3 * i] = (self
                    .copy_memory_buffer
                    .pread_with::<u8>(stride + offset_rgb + 2, scroll::LE)
                    .unwrap()
                    / 255) as f32;
                self.color_memory_buffer[3 * i + 1] = (self
                    .copy_memory_buffer
                    .pread_with::<u8>(stride + offset_rgb + 1, scroll::LE)
                    .unwrap()
                    / 255) as f32;
                self.color_memory_buffer[3 * i + 2] = (self
                    .copy_memory_buffer
                    .pread_with::<u8>(stride + offset_rgb, scroll::LE)
                    .unwrap()
                    / 255) as f32;
            }

            if offset_intensity != 0 && use_intensity_channel {
                let intensity: f32 = self
                    .copy_memory_buffer
                    .pread_with::<f32>(stride + offset_intensity, scroll::LE)
                    .unwrap()
                    / 255.0;
                let normalized_intensity = if intensity < 360.0 {
                    intensity
                } else {
                    360.0
                };

                if use_rainbow {
                    let color: (u8, u8, u8) = HSL {
                        h: normalized_intensity as f64,
                        s: 1.0,
                        l: 0.5
                    }.to_rgb();

                    self.color_memory_buffer[3 * i] = color.2 as f32;
                    self.color_memory_buffer[3 * i + 1] = color.1 as f32;
                    self.color_memory_buffer[3 * i + 2] = color.0 as f32;
                } else {
                    self.color_memory_buffer[3 * i] = normalized_intensity * (255.0/360.0);
                    self.color_memory_buffer[3 * i + 1] = normalized_intensity * (255.0/360.0);
                    self.color_memory_buffer[3 * i + 2] = normalized_intensity * (255.0/360.0);
                }
            }
        }
    }
}
