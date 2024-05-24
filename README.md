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

## Algorithm

The algorithm is quite simple. First, we create a Sketch that constists of primitives. Primitives can also reference each other. Take for example a line that is defined by two points. If one of the points moves, the line should also move. This is done by creating a reference to the point in the line. The line will then always calculate its position based on the points.

Now we take all these parameters and put them into a big vector $q$. This vector is the current state of our sketch. For example

$$
q = [x_1, y_1, x_2, y_2, x_3, y_3, r_1, x_4, y_4, x_5, y_5, r_2]
$$

## Todos

- [x] When adding a constraint, check that all primitives are already in the sketch

## Usage
