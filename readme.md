# HSFC-Generator

A WIP Hilbert Space-Filling Curve generator with a built-in OBJ generator for rendering the curves

Intended for use as a library, but contains a main.rs file with example usage.

## Submodules:

### Hilbert-Curve

This module contains functionality for generating the Cartesian coordinates of the vertices in a Hilbert Space-Filling Curve in both 2D and 3D.

It also contains utility functions for converting the output vertices into different formats:

- (x,y,z) in binary
- (x,y,z) in decimal

### Linear_Path

This module contains functionality for generating OBJ files from the cartesian coordinates of a Hilbert Curve.

These are useful for visualizing the output of the Hilbert Curve generator, and can be readily rendered by uploading to an online OBJ file renderer.
