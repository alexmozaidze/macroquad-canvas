# Notice

This is a hard fork of the original
[macroquad-canvas-2d](https://git.sr.ht/~nik_codes/macroquad-canvas); the only thing that I didn't
touch is the math involving aligning the canvas, everything else was changed.

# Description

**macroquad-canvas** is a simple resolution-handling library for
[Macroquad](https://github.com/not-fl3/macroquad) that allows you to focus on making your games with
fixed resolution. If you're making a pixel game, then this library is for you!

# How To Use

**macroquad-canvas** is easy to use and setup, the only thing you'll need to import is `Canvas2D`.

```rust,no_run
use macroquad_canvas::Canvas2D;
```

Then simply create the canvas and you're good to go!

```rust,no_run
# use macroquad_canvas::Canvas2D;
let canvas = Canvas2D::new(800_f32, 600_f32);
```

and here's how to draw the canvas

```rust,no_run
# #![no_main]
# use macroquad::prelude::*;
# use macroquad_canvas::Canvas2D;
#
# #[macroquad::main("Basic usage")]
# async fn main() {
# let canvas = Canvas2D::new(800_f32, 600_f32);
loop {
    set_camera(&canvas.camera);

    // Draw inside canvas...

    set_default_camera();

    canvas.draw();

    # return;
    next_frame().await
}
# }
```

That's all you need to know to get started! For more examples check out the
[examples](https://github.com/alexmozaidze/macroquad-canvas/tree/main/examples) folder.
