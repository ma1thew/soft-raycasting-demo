# soft-raycasting-demo

A simple (and slightly messy) software ray casting renderer, written in Rust.
It uses [rust_minifb](https://github.com/emoon/rust_minifb) to create a window within which to render and capture input.
Here's some of it's features:

* A cell based layout system.
* Affine texture mapped walls, floors and ceilings, with each cell able to have unique textures.
* A simple font renderer, used for an FPS display.
* Primitive BMP parsing for fonts and textures.
* Simple lighting effect based on the alignment of a cell's walls.
* Sprite rendering.
* Adjustable camera height.
* A 2D Z-buffer.
* Thin wall support, including transparency.
* Per-tile fog (but not volumetric).

Based on [Lode Vandevenne's graphics tutorials](https://lodev.org/cgtutor/).
All code is WTFPL licensed, and the assets in res/textures are the property of ID Software.
