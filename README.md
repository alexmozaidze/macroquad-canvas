[![Crates.io](https://img.shields.io/crates/v/macroquad-canvas?style=flat)](https://crates.io/crates/macroquad-canvas)
[![Lib.rs](https://img.shields.io/crates/v/macroquad-canvas?color=%2384f&label=lib.rs)](https://lib.rs/crates/macroquad-canvas)
[![docs.rs](https://img.shields.io/docsrs/macroquad-canvas?style=flat)](https://docs.rs/macroquad-canvas/0.3.2/macroquad_canvas/)
[![License](https://img.shields.io/crates/l/macroquad-canvas)](https://gitlab.com/alexmozaidze/macroquad-canvas/-/blob/main/LICENSE)
![Maintenance](https://img.shields.io/maintenance/yes/2023)

# Macroquad Canvas

*This is a hard fork of the original
[macroquad-canvas-2d](https://git.sr.ht/~nik_codes/macroquad-canvas)*

A simple resolution-handling library for
[Macroquad](https://github.com/not-fl3/macroquad) that allows you to focus on making your games with
fixed resolution.

If you're making a pixel game, then this library is for you!

## How To Use

To use this library, import `Canvas2D`

```rust
use macroquad_canvas::Canvas2D;
```

Then simply create the canvas

```rust
let canvas = Canvas2D::new(800_f32, 600_f32);
```

And here's how to draw the canvas

```rust
loop {
    set_camera(&canvas.camera);

    // Draw inside canvas...

    set_default_camera();

    canvas.draw();

    next_frame().await
}
```

That's all you need to know to get started!  
For more examples, check out the
[examples](https://gitlab.com/alexmozaidze/macroquad-canvas/-/tree/main/examples) folder.
