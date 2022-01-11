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

```rust
use macroquad_canvas::Canvas2D;
```

Then simply create the canvas and you're good to go!

```rust
let width = 800;
let height = 600;
let canvas = Canvas2D::new(width, height);
```

and here's how to draw the canvas

```rust
loop {
    set_camera(&canvas.camera);

    // Draw inside canvas...

    set_default_camera();

    canvas.draw_default();

    next_frame().await
}
```

That's all you need to know to get started! For more examples check out the
[examples](https://github.com/alexmozaidze/macroquad-canvas/tree/main/examples) folder.

# TODO

*Sorted in order of execution*

[ ] Function to transform canvas coordinates to screen coordinates.  
[ ] Simple post processing effects.  
[ ] Shaders.  
[ ] Add more examples.
