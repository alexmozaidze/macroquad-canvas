#![forbid(unsafe_code)]
#![warn(
    clippy::cargo_common_metadata,
    clippy::cloned_instead_of_copied,
    clippy::equatable_if_let,
    clippy::if_then_some_else_none,
    clippy::lossy_float_literal,
    clippy::map_unwrap_or,

    missing_docs,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
)]

#![doc = include_str!("../README.md")]

//---------------------------------------------------------------------

use macroquad::prelude::*;
use std::ops::{Deref, DerefMut};

//---------------------------------------------------------------------

/// Fixed size 2D canvas
///
/// # Description
///
/// `Canvas2D` is basicaly a wrapper around `Camera2D` with convinience
/// methods to make life easier.
///
/// # Note
///
/// `Deref` and `DerefMut` traits are implemented; this means you can access camera's fields and
/// methods directly from canvas like so:
///
/// ```rust,no_run
/// # use macroquad_canvas::Canvas2D;
/// let canvas = Canvas2D::new(800_f32, 600_f32);
///
/// // These lines do the same thing
/// println!("{}", canvas.zoom);
/// println!("{}", canvas.camera.zoom);
/// ```
///
/// # Implementation Detail
///
/// There's a [bug](https://github.com/not-fl3/macroquad/issues/171#issuecomment-880601087) that
/// mirrors render target on the Y axis, as a workaround, the render target gets flipped vertically.
#[derive(Clone, Copy)]
pub struct Canvas2D {
    /// The wrapped `Camera2D` necessary for all the calculations
    pub camera: Camera2D,
}

impl Deref for Canvas2D {
    type Target = Camera2D;

    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl DerefMut for Canvas2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}

impl Canvas2D {
    /// Creates a new canvas.
    ///
    /// # Why does it take floats instead of integers?
    ///
    /// The reason it takes floats and not integers is because
    /// Macroquad uses floats in (almost) all of its functions.
    pub fn new(width: f32, height: f32) -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
        camera.render_target = Some(render_target(width as u32, height as u32));
        // Flip vertically
        camera.zoom.y = -camera.zoom.y;

        Self { camera }
    }

    /// Draws canvas to the screen.
    #[inline]
    pub fn draw(&self) {
        self.draw_ex(screen_width(), screen_height());
    }

    /// Draws canvas with target width/height.
    pub fn draw_ex(&self, target_width: f32, target_height: f32) {
        let (left_padding, top_padding, dimensions) =
            self.get_size_and_padding(target_width, target_height);

        draw_texture_ex(
            *self.get_texture(),
            left_padding,
            top_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dimensions),
                ..Default::default()
            },
        );
    }

    /// Returns canvas width.
    #[inline]
    pub fn width(&self) -> f32 {
        self.get_texture().width()
    }

    /// Returns canvas height.
    #[inline]
    pub fn height(&self) -> f32 {
        self.get_texture().height()
    }

    /// Returns mouse position on the canvas
    #[inline]
    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position_ex(screen_width(), screen_height())
    }

    /// Returns mouse position with target width/height.
    pub fn mouse_position_ex(&self, target_width: f32, target_height: f32) -> (f32, f32) {
        // Mouse position on screen
        let (mouse_x, mouse_y) = mouse_position();

        let scale = self.get_min_scale_factor(target_width, target_height);

        // Mouse position on canvas
        let virtual_mouse_x = (mouse_x - (target_width - (self.width() * scale)) * 0.5) / scale;
        let virtual_mouse_y = (mouse_y - (target_height - (self.height() * scale)) * 0.5) / scale;

        (
            virtual_mouse_x.clamp(0.0, self.width()).floor(),
            virtual_mouse_y.clamp(0.0, self.height()).floor(),
        )
    }

    /// Returns a reference to the canvas texture.
    #[inline]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_texture(&self) -> &Texture2D {
        &self.render_target.as_ref().unwrap().texture
    }

    /// Returns a mutable reference to the canvas texture.
    #[inline]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_texture_mut(&mut self) -> &mut Texture2D {
        &mut self.render_target.as_mut().unwrap().texture
    }

    /// Calculate size of the canvas so it can fit inside of the target.
    pub fn get_size(&self, target_width: f32, target_height: f32) -> Vec2 {
        // Get the min scale factor
        let min_scale_factor: f32 = self.get_min_scale_factor(target_width, target_height);

        // Calculate windows new size
        let new_width: f32 = self.width() * min_scale_factor;
        let new_height: f32 = self.height() * min_scale_factor;

        Vec2::new(new_width, new_height)
    }

    /// Returns padding of the canvas.
    ///
    /// # Note
    ///
    /// Internally it uses [`Canvas2D::get_size_and_padding`] and simply drops the size,
    /// so if you also need size, consider just using [`Canvas2D::get_size_and_padding`]
    pub fn get_padding(&self, target_width: f32, target_height: f32) -> (f32, f32) {
        let (left_padding, top_padding, _) = self.get_size_and_padding(target_width, target_height);

        (left_padding, top_padding)
    }

    /// Returns size and padding of the canvas. Used in [`Canvas2D::draw_ex`] for fitting the canvas
    /// on the screen and for centering.
    pub fn get_size_and_padding(&self, target_width: f32, target_height: f32) -> (f32, f32, Vec2) {
        let new_size: Vec2 = self.get_size(target_width, target_height);

        // Calculate padding
        let left_padding: f32 = (target_width - new_size.x) / 2.0;
        let top_padding: f32 = (target_height - new_size.y) / 2.0;

        (left_padding, top_padding, new_size)
    }

    /// Returns scale factors.
    pub fn get_scale_factor(&self, target_width: f32, target_height: f32) -> (f32, f32) {
        (target_width / self.width(), target_height / self.height())
    }

    /// Returns scale factors' minimum value.
    pub fn get_min_scale_factor(&self, target_width: f32, target_height: f32) -> f32 {
        let (scale_width, scale_height) = self.get_scale_factor(target_width, target_height);

        f32::min(scale_width, scale_height)
    }
}
