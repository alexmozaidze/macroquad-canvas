/*
  Copyright 2021 Nicolas Cesar Sabbatini Vrech

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use macroquad::prelude::*;

pub struct Canvas2D {
    canvas: RenderTarget,
    camera: Camera2D,
    pub width: f32,
    pub height: f32,
}

impl Canvas2D {
    /// Create a new canvas
    pub fn new(width: f32, height: f32) -> Self {
        let canvas = render_target(width as u32, height as u32);
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
        camera.render_target = Some(canvas);
        Canvas2D {
            canvas,
            camera,
            width,
            height,
        }
    }

    /// Draw inside of the canvas
    pub fn draw_inside_canvas(&self, f: &dyn Fn() -> ()) {
        set_camera(&self.camera);
        f();
        set_default_camera();
    }

    /// Calculate size and padding of the canvas so it can fit inside of the target and its position is in the center
    pub fn calculate_size_and_padding(
        &self,
        target_width: f32,
        target_height: f32,
    ) -> (f32, f32, Vec2) {
        // Calculate scale factors
        let scale_factor_w: f32 = target_width / self.width;
        let scale_factor_h: f32 = target_height / self.height;

        // Get the min scale factor
        let min_scale_factor: f32 = f32::min(scale_factor_w, scale_factor_h);

        // Calculate windows new size
        let new_width: f32 = self.width * min_scale_factor;
        let new_height: f32 = self.height * min_scale_factor;

        // Calculate padding
        let left_padding: f32 = (target_width - new_width) / 2.0;
        let top_padding: f32 = (target_height - new_height) / 2.0;

        (left_padding, top_padding, Vec2::new(new_width, new_height))
    }

    /// Calculate size of the canvas so it can fit inside of the target
    pub fn calculate_size(&self, target_width: f32, target_height: f32) -> Vec2 {
        // Calculate scale factors
        let scale_factor_w: f32 = target_width / self.width;
        let scale_factor_h: f32 = target_height / self.height;

        // Get the min scale factor
        let min_scale_factor: f32 = f32::min(scale_factor_w, scale_factor_h);

        // Calculate windows new size
        let new_width: f32 = self.width * min_scale_factor;
        let new_height: f32 = self.height * min_scale_factor;

        Vec2::new(new_width, new_height)
    }

    /// Get a reference of the canvas texture
    pub fn get_texture(&self) -> &Texture2D {
        &self.canvas.texture
    }
}
