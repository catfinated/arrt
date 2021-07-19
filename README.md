# arrt
**This is a personal project used for learning. Pull requests will mostly be ignored.**

A Rust Ray Tracer. This is my hobby project for learning Rust. This is  mostly based on a C++ ray
tracer I wrote many years ago in a grad school class where the curriculum was based on the book
"Ray Tracing from the Ground Up" by Kevin Suffern.

This ray tracer is exteremly basic right now it only supports two types of objects: spheres and
smf meshes and the only optimization done is using a bounding volume heirarchy. Here are some examples
of what it can currently prodcue:

![Example 1](docs/scene.png)
![Example 2](docs/scene2.png)

## TODO

* antialiasing
* instancing
* reflections and refractions
* bezeir pathes/curves
