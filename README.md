# arrt
**This is a personal project used for learning. Pull requests will mostly be ignored.**

A Rust Ray Tracer. This is my hobby project for learning Rust. This is  mostly based on a C++ ray
tracer I wrote many years ago in a grad school class where the curriculum was based on the book
"Ray Tracing from the Ground Up" by Kevin Suffern.

## Features

This ray tracer is exteremly basic right now but the following features are included:

* Triangle Meshes
  * Simple Mesh Format
  * Superquadrics
  * Bezier Patches
  * Instancing
* Spheres and planes
* Bounding volume hiearchy based on axis aligned bounding boxes
* Phong/Hall shading with point and spot light sources
  * Shadows
  * Reflection
  * Refraction
* Adaptive super sampling (1 - 2x)
* Parallelized tracing and rendering with rayon

Here are some examples of what it can currently prodcue:

|                               |                                     |
| ----------------------------- | ----------------------------------- |
| ![Example 1](docs/scene.png)  | ![Example 2](docs/bunnies.png)      |
| ![Example 3](docs/cow.png)    | ![Example 4](docs/cow2.png)         |
| ![Example 5](docs/dragon.png) | ![Example 6](docs/glass_teapot.png) |
| ![Example 7](docs/bpsq.png)   |                                     |


## TODO

* area lights
* texture mapping
* perlin noise
