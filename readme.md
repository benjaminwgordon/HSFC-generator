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
of generated points will typically be n^2, where n is a non-negative integer.

The algorithm used to calculate these points may not be optimal,
and I intend to return to this in the future if BRGC generation
becomes the performance bottleneck of hilbert curve generation.

### Linear_Path

This represents a list of cartesian coordinates and a traversal path

For all reasonable cases, the traversal path will just be the set of
the first n sequential non-negative integer pairs, e.g. [(0,1), (1,2),
(2,3), (...), (n-2, n-1)]. Due to this, I may remove the path field
from this struct in the future.

This module has a method for constructing a Linear_Path from an iterator
of u32 items, and other data types for the linear path could be introduced
in the future.

It also contains methods for exporting the linear path data to CSV and OBJ for
visualization.
