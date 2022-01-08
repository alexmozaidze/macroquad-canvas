#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    // restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_self_imports,
    clippy::unseparated_literal_suffix,

    // nursery lints
    clippy::debug_assert_with_mut_call,
    clippy::equatable_if_let,
    clippy::fallible_impl_from,
    clippy::option_if_let_else,
    clippy::path_buf_push_overwrite,
    clippy::use_self,

    // pedantic lints
    clippy::cloned_instead_of_copied,
    clippy::filter_map_next,
    clippy::if_not_else,
    clippy::from_iter_instead_of_collect,
    clippy::implicit_clone,
    clippy::inefficient_to_string,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::macro_use_imports,
    clippy::manual_assert,
    clippy::manual_ok_or,
    clippy::map_unwrap_or,
    clippy::match_bool,
    clippy::mut_mut,
    clippy::needless_bitwise_bool,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::ptr_as_ptr,
    clippy::redundant_closure_for_method_calls,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::unicode_not_nfc,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::unused_async,
    clippy::used_underscore_binding,
    clippy::verbose_bit_mask,

    // other
    missing_docs,
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
)]

//---------------------------------------------------------------------

use macroquad::prelude::*;
use std::ops::{Deref, DerefMut};

//---------------------------------------------------------------------

/// This struct represents the canvas, it's basicaly a wrapper around `Camera2D` with convinience
/// methods to make life easier.
///
/// # Note
///
/// `Deref` and `DerefMut` are implemented, which means you can access fields/methods from
/// `Camera2D` like so
///
/// ```rust
/// let canvas = Canvas2D::new(800, 600);
/// println!("{}", canvas.zoom); // Prints camera zoom
/// ```
///
/// # Implementation Detail
///
/// There's a bug that mirrors render target on the Y axis (see
/// <https://github.com/not-fl3/macroquad/issues/171#issuecomment-880601087>), as a workaround,
/// the render target gets flipped vertically.
pub struct Canvas2D {
    /// Contains the camera which contains the render texture and other things.
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
    /// Creates a new canvas
    ///
    /// # Example
    ///
    /// ```rust
    /// let canvas = Canvas2D::new(800, 600);
    /// ```
    pub fn new(width: impl Into<f32>, height: impl Into<f32>) -> Self {
        let width = width.into();
        let height = height.into();

        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
        camera.render_target = Some(render_target(width as u32, height as u32));
        // Flip vertically
        camera.zoom.y = -camera.zoom.y;

        Self { camera }
    }

    /// Returns a reference of the canvas texture
    ///
    /// # Panics
    ///
    /// If the render target is missing.
    pub fn get_texture(&self) -> &Texture2D {
        &self.render_target.as_ref().unwrap().texture
    }

    /// Returns canvas width.
    ///
    /// # Panics
    ///
    /// If the render target is missing.
    pub fn width(&self) -> f32 {
        self.get_texture().width()
    }

    /// Returns canvas height.
    ///
    /// # Panics
    ///
    /// If the render target is missing.
    pub fn height(&self) -> f32 {
        self.get_texture().height()
    }

    /// Calculate size and padding of the canvas so it can fit inside of the target and its position
    /// is in the center.
    ///
    /// # Example
    ///
    /// ```rust
    /// let (left_padding, top_padding, dimensions) =
    ///     self.calculate_size_and_padding(screen_width(), screen_height());
    /// ```
    pub fn calculate_size_and_padding(
        &self,
        target_width: impl Into<f32>,
        target_height: impl Into<f32>,
    ) -> (f32, f32, Vec2) {
        let target_width = target_width.into();
        let target_height = target_height.into();

        let new_size: Vec2 = self.calculate_size(target_width, target_height);

        // Calculate padding
        let left_padding: f32 = (target_width - new_size.x) / 2.0;
        let top_padding: f32 = (target_height - new_size.y) / 2.0;

        (left_padding, top_padding, new_size)
    }

    /// Calculate size of the canvas so it can fit inside of the target.
    ///
    /// # Example
    ///
    /// ```rust
    /// canvas.calculate_size(screen_width(), screen_height());
    /// ```
    pub fn calculate_size(
        &self,
        target_width: impl Into<f32>,
        target_height: impl Into<f32>,
    ) -> Vec2 {
        let target_width = target_width.into();
        let target_height = target_height.into();

        // Calculate scale factors
        let scale_factor_w: f32 = target_width / self.width();
        let scale_factor_h: f32 = target_height / self.height();

        // Get the min scale factor
        let min_scale_factor: f32 = f32::min(scale_factor_w, scale_factor_h);

        // Calculate windows new size
        let new_width: f32 = self.width() * min_scale_factor;
        let new_height: f32 = self.height() * min_scale_factor;

        Vec2::new(new_width, new_height)
    }

    /// Draws canvas in the center of the screen.
    ///
    /// # Example
    ///
    /// Draw canvas according to the window:
    ///
    /// ```rust
    /// canvas.draw(None);
    /// ```
    ///
    /// Draw canvas with custom target dimensions:
    ///
    /// ```rust
    /// canvas.draw((800.0, 600.0));
    /// ```
    pub fn draw(
        &self,
        target_dim: impl Into<Option<(f32, f32)>>,
    ) {
        let (target_width, target_height) =
            target_dim.into().unwrap_or((screen_width(), screen_height()));

        let (left_padding, top_padding, dimensions) =
            self.calculate_size_and_padding(target_width, target_height);

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
}
