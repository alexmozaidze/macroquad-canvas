use macroquad::prelude::*;
use macroquad_canvas_2d::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn win_config() -> Conf {
    Conf {
        window_title: "MacroPong".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(win_config)]
async fn main() {
    let canvas = Canvas2D::new(WIDTH as f32, HEIGHT as f32);

    let ferris = load_texture("examples/assets/ferris-small.png")
        .await
        .unwrap();

    loop {
        // Draw inside canvas
        canvas.draw_inside_canvas(&|| {
            // Clear background
            clear_background(WHITE);
            // Draw something
            // Top left
            draw_rectangle(0.0, 0.0, 60.0, 60.0, RED);
            // Top right
            draw_rectangle(WIDTH as f32 - 60.0, 0.0, 60.0, 60.0, GRAY);
            // Bottom left
            draw_rectangle(0.0, HEIGHT as f32 - 60.0, 60.0, 60.0, GREEN);
            // Bottom right
            draw_rectangle(WIDTH as f32 - 60.0, HEIGHT as f32 - 60.0, 60.0, 60.0, BLUE);
            // Center
            draw_texture(
                ferris,
                (WIDTH as f32 - ferris.width()) / 2.,
                (HEIGHT as f32 - ferris.height()) / 2.,
                WHITE,
            );
        });

        clear_background(BLACK);

        // Get canvas dimensions and padding
        let (left_padding, top_padding, dimensions) =
            canvas.calculate_size_and_padding(screen_width(), screen_height());

        // Draw canvas on screen
        draw_texture_ex(
            *canvas.get_texture(),
            left_padding,
            top_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dimensions),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
