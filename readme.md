# Hilbert-Curve-Generator

A Hilbert Space-Filling Curve Coordinate Generator. Can produce cartesian coordinates for the vertices of a Hilbert Space-Filling Curve in 2 or 3 dimensions using the Skilling Transform Method.

## Project Structure:

### Hilbert-Curve

This module contains functionality for generating the Cartesian coordinates of the vertices in a Hilbert Space-Filling Curve in both 2D and 3D.

It also contains utility functions for converting the output vertices into different formats:

- (x,y,z) in binary
- (x,y,z) in decimal

### BRGC

Brgc is the Binary Reflected Gray Code Iterator

This module exposes an iterator that counts using the Binary Reflected Gray Code.
This counting system has unique properties that make it the ideal starting point 
for generating the coordinates of a Hilbert Curve.

The iterator exposed by this module can be used to generate
points in a BRGC of any magnitude, although for the typical purposes
of generating square hilbert curves and cubic hilbert cubes, the number
of generated points will typically be p^n, where n is the number of dimensions
for the resulting cartesian coordinates (2D and 3D currently supported), and p
is the number of data bits for each coordinate (each side of your square/cube
will have 2^p vertices)

## Skilling_Transform

This module contains functions for applying the Skilling Transform to an existing
Vec<u32> of BRGC.

The Skilling Transform is an algorithm for generating the cartesian coordinates of
vertices in a Hilbert curve without using the traditional recursive method.

I've optimized for readability and ease of understanding in the code,
since the algorithm is not immediately clear. There was a good deal of trial
and error in implementing this algorithm. If I spend more time on this project,
I'll try replacing all the string manipulation with bit manipulation and benchmark
the speed difference.
