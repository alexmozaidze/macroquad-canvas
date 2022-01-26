/*
 * This example shows how to get mouse position inside canvas
 */

use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

const WIDTH: f32 = 800_f32;
const HEIGHT: f32 = 600_f32;

const FONT_SIZE: f32 = 24.0;

#[macroquad::main("Woo-Woo Virtual Mouse!")]
async fn main() {
    // Creating the canvas
    let canvas = Canvas2D::new(WIDTH, HEIGHT);

    loop {
        // Set focus on canvas camera
        set_camera(&canvas.camera);

        clear_background(WHITE);

        // Mouse position
        let (screen_mouse_x, screen_mouse_y) = mouse_position();

        // Mouse position inside canvas
        let (canvas_mouse_x, canvas_mouse_y) = canvas.mouse_position();

        // Draw a circle at mouse position
        draw_circle(canvas_mouse_x, canvas_mouse_y, 20.0, DARKGRAY);

        // Print mouse position
        draw_text(
            &format!("screen: X {} Y {}", screen_mouse_x, screen_mouse_y), // Text
            0.0,                                                           // X
            FONT_SIZE,                                                     // Y
            FONT_SIZE,                                                     // Font size
            BLACK,                                                         // Color
        );
        draw_text(
            &format!("canvas: X {} Y {}", canvas_mouse_x, canvas_mouse_y), // Text
            0.0,                                                           // X
            FONT_SIZE * 2.0,                                               // Y
            FONT_SIZE,                                                     // Font size
            BLACK,                                                         // Color
        );

        // Return to the default camera
        set_default_camera();

        clear_background(BLACK);

        // Drawing the canvas on the screen
        canvas.draw();

        next_frame().await
    }
}
