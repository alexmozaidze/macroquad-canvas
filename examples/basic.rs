use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

const WIDTH: f32 = 800_f32;
const HEIGHT: f32 = 600_f32;

fn win_config() -> Conf {
    Conf {
        window_title: "Basic usage".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(win_config)]
async fn main() {
    // Creating the canvas
    let canvas = Canvas2D::new(WIDTH, HEIGHT);

    // Loading 🦀 texture
    let ferris = load_texture("examples/assets/ferris-small.png")
        .await
        .unwrap();

    loop {
        // Set focus on canvas camera
        set_camera(&canvas.camera);

        clear_background(WHITE);

        // Draw rectangles
        draw_rectangle(0.0, 0.0, 60.0, 60.0, RED); // Top left
        draw_rectangle(WIDTH - 60.0, 0.0, 60.0, 60.0, GRAY); // Top right
        draw_rectangle(0.0, HEIGHT - 60.0, 60.0, 60.0, GREEN); // Bottom left
        draw_rectangle(WIDTH - 60.0, HEIGHT - 60.0, 60.0, 60.0, BLUE); // Bottom right

        // Draw 🦀
        draw_texture(
            ferris,
            (WIDTH - ferris.width()) / 2.0,
            (HEIGHT - ferris.height()) / 2.0,
            WHITE,
        );

        // Return to the default camera
        set_default_camera();

        clear_background(BLACK);

        // Drawing the canvas on the screen
        canvas.draw_default();

        next_frame().await
    }
}
