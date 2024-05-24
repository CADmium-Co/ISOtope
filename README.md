# ISOtope

The **I**terative **SO**lver for 2D Sketches. This project is a dependency free gradient descent based solver. It works by minimizing the energy of virtual springs that represent the constraints.

- Keeps it simple and stupid. Most PCs are more than powerful enough to solve small sketches in real time. We don't need to overcomplicate things.
- Each Sketch is a list of primitives and constraints.
- Supported primitives
    - [x] Point
    - [x] Line
    - [x] Circle
    - [x] Arc
- Each constraint is a virtual spring and has a function to calculate its energy (or call it loss function) and the gradient. Current constraints are:
    - [ ] Distance
    - [ ] Angle
    - [x] Horizontal
    - [x] Vertical
    - [x] Coincident
    - [ ] Tangent
    - [ ] Concentric
    - [ ] Equal
    - [ ] Parallel
    - [ ] Perpendicular
    - [ ] Fix

## Todos

- [x] When adding a constraint, check that all primitives are already in the sketch

## Usage
