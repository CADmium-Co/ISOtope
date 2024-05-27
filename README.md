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
    - [x] Distance
    - [x] Angle
    - [x] Horizontal
    - [x] Vertical
    - [x] Coincident
    - [x] Parallel
    - [x] Perpendicular
    - [x] Fix
    - [x] Equal length
    - [x] Concentric (can be done by using same ref for center points for circle and arcs)
    - [ ] Tangent
- All gradients are checked with finite differences to ensure correctness

## Algorithm

### Modeling

The algorithm is quite simple. First, we create a Sketch that constists of primitives. Primitives can also reference each other. Take for example a line that is defined by two points. If one of the points moves, the line should also move. This is done by creating a reference to the point in the line. The line will then always calculate its position based on the points.

Consider for example a simple sketch consisting of three points $p_a$, $p_b$ and $p_c$, and two lines $l_1$ and $l_2$.

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
l_1 = \overline{p_a p_b}
$$

$$
l_2 = \overline{p_b p_c}
$$

Now we take all these parameters and put them into a big vector $q$. This vector is the current state of our sketch. 

$$
q = [x_1, y_1, x_2, y_2, x_3, y_3]
$$

This is great, now all we have to do is to find the value for $q$ that satisfies all the constraints. We do this by minimizing the energy of the constraints. The energy of a constraint is a function that takes the current state of the sketch and returns a scalar value. This value is the energy of the constraint. The gradient of the energy is a vector that points in the direction of the steepest ascent. We can use this gradient to update the state of the sketch. 

If you want to have a visual interpretation, think of physical springs. [The energy of a spring is given by](https://en.wikipedia.org/wiki/Hooke%27s_law#Spring_energy)

$$
E = 0.5 * k * x^2
$$

We model all our constraints the same way. Take for example a coincident constraint.

$$
L(x_1, y_1, x_2, y_2) = 0.5 * {(x_1 - x_2)^2 + (y_1 - y_2)^2}
$$

Or a horizontal constraint

$$
L(x_1, y_1, x_2, y_2) = 0.5 * (y_1 - y_2)^2
$$

Some constraints are more complex, like a distance constraint, but in the end all of them are a function that map the current state of the sketch to a scalar value which we want to minimize.

### Problem formulation

We take all the constraints and put them into a big function $L(q)$ that takes the current state of the sketch and returns the sum of all the energies of the constraints. Then our goal is

$$
\min_q L(q)
$$

Luckily, we are not only provided with $L(q)$, but also with the gradient of $L(q)$, which is a vector that points in the direction of the steepest ascent. We can use this gradient to update the state of the sketch.

$$
\nabla L(q)
$$

### Solving via gradient descent

Now all we have to do is a simple gradient descent

$$
q_{t+1} = q_t - \alpha \sum_i \nabla L_i(q_t)
$$

Where $\alpha$ is the step size (typicall 0.001) and $\nabla L_i(q_t)$ is the gradient of the energy of the constraints. If we do this often enough, e.g. 100000 times, we will find a state of the sketch that satisfies all the constraints as much as possible. It ends up in the same phyical state as a spring system would. Even though 100000 times sounds a like a lot, it is not for a modern PC. It just takes a couple of milliseconds.

### Solving with BFGS

Gradient descent is not the only way to solve this problem. There are many other solvers that are faster and more robust. One of them is the BFGS solver. It is a quasi-newton method that approximates the hessian of the loss function. This is a matrix that tells us how the gradient changes. It is a bit more complex, but also a lot faster. If you are interested in the details, check out the [wikipedia page](https://en.wikipedia.org/wiki/Broyden%E2%80%93Fletcher%E2%80%93Goldfarb%E2%80%93Shanno_algorithm).

BFGS solver is the default solver people should use. It is faster and more robust than gradient descent. Also, the solutions are much more accurate.

### Conflict resolution

In case of conflicting constraints, we can also figure out which constraints are the ones that actually cause the conflict. Constraints that are satisfied will have a energy/loss of 0.

$$
L_i(q) = 0
$$

constraints that are in conflict will have a non-zero energy/loss.

$$
L_i(q) > 0
$$

and can be highlighted to the user.

## Usage

Check out the examples folder at [src/examples](src/examples) for more examples.

```rust
#[test]
pub fn test_rectangle_rotated() {
    // Create a new empty sketch
    let sketch = Rc::new(RefCell::new(Sketch::new()));

    // Create four points
    let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
    let point_b = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
    let point_c = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
    let point_d = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));

    // Add the points to the sketch
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Point2(point_a.clone()))
        .unwrap();
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Point2(point_b.clone()))
        .unwrap();
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Point2(point_c.clone()))
        .unwrap();
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Point2(point_d.clone()))
        .unwrap();

    // Create four lines based on the points
    let line_a = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
    let line_b = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
    let line_c = Rc::new(RefCell::new(Line::new(point_c.clone(), point_d.clone())));
    let line_d = Rc::new(RefCell::new(Line::new(point_d.clone(), point_a.clone())));

    // Add the lines to the sketch
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Line(line_a.clone()))
        .unwrap();
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Line(line_b.clone()))
        .unwrap();
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Line(line_c.clone()))
        .unwrap();
    sketch
        .borrow_mut()
        .add_primitive(PrimitiveCell::Line(line_d.clone()))
        .unwrap();

    // Fix point a to origin
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
            FixPoint::new(point_a.clone(), Vector2::new(0.0, 0.0)),
        ))))
        .unwrap();

    // Constrain line_a and line_c to be horizontal
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
            HorizontalLine::new(line_a.clone()),
        ))))
        .unwrap();
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
            HorizontalLine::new(line_c.clone()),
        ))))
        .unwrap();

    // Constrain line_b and line_d to be vertical
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::VerticalLine(Rc::new(RefCell::new(
            VerticalLine::new(line_b.clone()),
        ))))
        .unwrap();
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::VerticalLine(Rc::new(RefCell::new(
            VerticalLine::new(line_d.clone()),
        ))))
        .unwrap();

    // Constrain the length of line_a to 2
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::HorizontalDistance(Rc::new(RefCell::new(
            HorizontalDistanceBetweenPoints::new(point_a.clone(), point_b.clone(), 2.0),
        ))))
        .unwrap();

    // Constrain the length of line_b to 3
    sketch
        .borrow_mut()
        .add_constraint(ConstraintCell::VerticalDistance(Rc::new(RefCell::new(
            VerticalDistanceBetweenPoints::new(point_a.clone(), point_d.clone(), 3.0),
        ))))
        .unwrap();

    // Now solve the sketch
    let solver = BFGSSolver::new(sketch.clone());
    solver.solve();

    println!("loss = {:?}", sketch.borrow_mut().get_loss());
    println!("point_a: {:?}", point_a.as_ref().borrow());
    println!("point_b: {:?}", point_b.as_ref().borrow());
    println!("point_c: {:?}", point_c.as_ref().borrow());
    println!("point_d: {:?}", point_d.as_ref().borrow());

    assert!((point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 1e-10);
    assert!((point_b.as_ref().borrow().data() - Vector2::new(2.0, 0.0)).norm() < 1e-10);
    assert!((point_c.as_ref().borrow().data() - Vector2::new(2.0, 3.0)).norm() < 1e-10);
    assert!((point_d.as_ref().borrow().data() - Vector2::new(0.0, 3.0)).norm() < 1e-10);
}
```

## Math cheat sheet

$$
\nabla (a + b) = \nabla a + \nabla b
$$

$$
\nabla (a \cdot b) = a \cdot \nabla b + b \cdot \nabla a
$$

$$
\nabla (\frac{a}{b}) = \frac{\nabla a \cdot b - a \cdot \nabla b}{b^2}
$$

$$
\nabla (f(a)) = \nabla f(a) \cdot \nabla a
$$

If $x$ and $y$ is a vector and $A$ is a matrix

$$
\nabla ||x||^2 = \nabla (x^T x) = 2 x^T
$$

$$
\nabla ||x|| = \nabla (\sqrt{x^T x}) = \frac{x^T}{\sqrt{x^T x}} = \frac{x^T}{||x||}
$$

$$
\nabla \frac{x}{||x||} = \nabla (\frac{x}{\sqrt{x^T x}}) = \frac{I \cdot ||x|| - x \cdot \frac{x^T}{||x||}}{x^T \cdot x} = \frac{1}{||x||} \big(I - \frac{x \cdot x^T}{x ^ T x}\big)
$$

$$
\nabla (Ax) = A
$$

$$
\nabla (y^T x) = y^T
$$
