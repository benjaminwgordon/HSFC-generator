# HSFC-Generator

A WIP Hilbert Space-Filling Curve generator and renderer

## Modules:

### BRGC

Brgc is the Binary Reflected Gray Code Iterator Implementation
This project generates the cartesian coordinates of each vertex in the
HSFC using a technique that begins with a list of the first n binary
numbers counted using the Binary Reflected Gray Code counting system.

The iterator exposed by this module can be used to generate
points in a BRGC of any magnitude, although for the typical purposes
of generating square hilbert curves and cubic hilbert cubes, the number
of generated points will typically be p^n, where n is the number of dimensions
for the resulting cartesian coordinates (2D and 3D currently supported), and p
is the number of data bits for each coordinate (each side of your square/cube
will have 2^p vertices)

## Skilling_Transform

This module contains functions for applying the Skilling Transform to an existing
Vector of BRGC u32's, and manipulating the output format for various uses.

I've optimized for readability and ease of understanding in the code,
since the algorithm is not immediately clear. There was a good deal of trial
and error in implementing this algorithm. If I spend more time on this project,
I'll try replacing all the string manipulation with bit manipulation and benchmark
the speed difference.

### Linear_Path

This represents a list of cartesian coordinates that are traversed linearly.

It contains methods for exporting the linear path data to CSV and OBJ for
visualization. During development, I used this module for evaluating the success
of my Skilling Transform implementation.
