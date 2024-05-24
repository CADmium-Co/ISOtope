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

Now we take all these parameters and put them into a big vector $q$. This vector is the current state of our sketch. Consider for example a simple sketch consisting of three points $p_a$, $p_b$ and $p_c$, and two lines $l_1$ and $l_2$. The vector $q$ would look like this:

$$
p_a = [x_1, y_1]
$$

$$
p_b = [x_2, y_2]
$$

$$
p_c = [x_3, y_3]
$$

$$
l_1 = \overleftrightarrow{p_a p_b}
$$

$$
l_2 = [x_2, y_2, x_3, y_3]
$$

$$
q = [x_1, y_1, x_2, y_2, x_3]
$$

## Todos

- [x] When adding a constraint, check that all primitives are already in the sketch

## Usage
