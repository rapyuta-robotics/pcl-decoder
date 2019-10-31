use scroll::Pread;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PCLDecoder {
    copy_memory_buffer: [u8; 60_000],
    position_memory_buffer: [f32; 10_000 * 3],
    color_memory_buffer: [f32; 10_000 * 3],
}

#[wasm_bindgen]
impl PCLDecoder {
    pub fn new() -> PCLDecoder {
        PCLDecoder {
            copy_memory_buffer: [0; 60_000],
            position_memory_buffer: [0.0; 10_000 * 3],
            color_memory_buffer: [0.0; 10_000 * 3],
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
                let normalized_intensity = if intensity / 255.0 < 255.0 {
                    intensity
                } else {
                    255.0
                };
                self.color_memory_buffer[3 * i] = if use_rainbow {
                    0.0
                } else {
                    normalized_intensity
                };
                self.color_memory_buffer[3 * i + 1] = normalized_intensity;
                self.color_memory_buffer[3 * i + 2] = if use_rainbow {
                    0.0
                } else {
                    normalized_intensity
                };
            }
        }
    }
}
