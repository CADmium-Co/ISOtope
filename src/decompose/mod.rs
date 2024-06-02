use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use geo::Polygon;
use geo::{Contains as _, InteriorPoint as _};

use crate::primitives::PrimitiveCell;
use crate::sketch::Sketch;

use self::face::Face;
use self::ring::Ring;
use self::segment::Segment;

pub mod face;
pub mod ring;
pub mod segment;

pub fn decompose_sketch(sketch: &Sketch) -> Vec<Face> {
    // A primitive is now ether a Circle, Line, or Arc. Points can be ignored.
    // Now chain all consecutive primitives that are connected into a ring.
    // - Two primitives are connected if the end of the first primitive is the start of the second primitive.
    // For now, assume there is only one ring in the sketch, such that the construction of the faces is simple.
    // For the future, we will need a more complex algorithm that can handle multiple rings. But for the MVP, this is sufficient.

    find_faces(sketch).0
}

pub fn merge_faces(faces: Vec<Face>) -> Vec<Face> {
    // check whether each of these new faces should returned or not by picking a
    // random point on the new face and then checking every one of the original faces
    // to see if it contains that point. If so, we can keep that new face
    let mut faces_to_remove: Vec<usize> = vec![];
    let old_faces_as_polygons: Vec<Polygon> = faces
        .iter()
        .map(|face| face.as_polygon())
        .collect::<Vec<_>>();
    for (new_face_idx, face) in faces.iter().enumerate() {
        let as_geo_polygon = face.as_polygon();

        #[allow(clippy::expect_used)]
        let random_point_on_face = as_geo_polygon
            .interior_point()
            .expect("Every polygon should be able to yield an interior point");

        let mut located = false;
        for old_face in old_faces_as_polygons.iter() {
            if old_face.contains(&random_point_on_face) {
                // this means the old face contains the random point on the new face
                // so we can keep this new face
                located = true;
                break;
            }
        }
        if !located {
            faces_to_remove.push(new_face_idx);
        }
    }

    // remove the faces that we don't want
    faces_to_remove.sort();
    faces_to_remove.reverse();

    let mut merged_faces = faces.clone();

    for face_to_remove in faces_to_remove {
        merged_faces.remove(face_to_remove);
    }

    merged_faces
}

pub fn find_faces(sketch: &Sketch) -> (Vec<Face>, Vec<Segment>) {
    let (rings, unused_segments) = find_rings(sketch);
    let mut faces: Vec<Face> = rings.iter().map(|r| Face::from_ring(r.clone())).collect();

    if rings.is_empty() {
        return (faces, unused_segments);
    }

    // this next block of code converts everything to Polygons just so we can
    // determine what faces contain which other faces. It's a bit of a waste
    // because geo is a relatively heavy dependency and we don't need all of it
    let polygons: Vec<Polygon> = rings.iter().map(|r| r.as_polygon()).collect();
    // they are already sorted from smallest to largest area - self.find_rings does this
    let mut what_contains_what: Vec<(usize, usize)> = vec![];

    for (smaller_polygon_index, smaller_polygon) in
        polygons[..polygons.len() - 1].iter().enumerate()
    {
        for (bigger_polygon_index, bigger_polygon) in
            polygons[smaller_polygon_index + 1..].iter().enumerate()
        {
            let inside = bigger_polygon.contains(smaller_polygon);

            if inside {
                what_contains_what.push((bigger_polygon_index, smaller_polygon_index));
                break;
            }
        }
    }

    // cool, now we know what faces contain which other faces. Let's just add the holes
    for (bigger_index, smaller_index) in what_contains_what {
        // TODO: Can this lookup fail?
        let smaller_face = faces[smaller_index].clone();
        faces[bigger_index].add_hole(smaller_face)
    }

    (faces, unused_segments)
}

pub fn find_rings(sketch: &Sketch) -> (Vec<Ring>, Vec<Segment>) {
    let init_segments: Vec<Segment> = sketch
        .primitives()
        .values()
        .filter_map(|p| match p {
            // We don't consider circles - we'll just add them to the rings directly (right?)
            PrimitiveCell::Line(l) => Some(Segment::Line(l.borrow().clone())),
            PrimitiveCell::Arc(a) => Some(Segment::Arc(a.borrow().clone())),
            _ => None,
        })
        .collect();

    let segments_reversed = init_segments.iter().map(|s| s.reverse());

    // We consider all given segments and their reversed counterparts
    let all_segments: Vec<Segment> = init_segments
        .iter()
        .cloned()
        .chain(segments_reversed)
        .collect();

    let mut used_indices: BTreeSet<usize> = BTreeSet::new();
    let mut new_rings: Vec<Vec<&Segment>> = vec![];

    for (segment_index, segment) in all_segments.iter().enumerate() {
        if used_indices.contains(&segment_index) {
            continue;
        }

        let mut new_ring_indices: Vec<(usize, &Segment)> = vec![];
        let start_point = segment.get_start();

        let mut next_segment_index = segment_index;
        let mut next_segment = segment;
        for _i in 1..all_segments.len() {
            new_ring_indices.push((next_segment_index, next_segment));

            if next_segment.get_end() == start_point {
                new_rings.push(new_ring_indices.iter().map(|x| x.1).collect());
                used_indices.extend(new_ring_indices.iter().map(|x| x.0));
                break;
            }

            (next_segment_index, next_segment) =
                match find_next_segment(&all_segments, next_segment, &used_indices) {
                    Some((index, segment)) => (index, segment),
                    None => break,
                };
        }
    }

    let unused_segments = init_segments
        .iter()
        .enumerate()
        .filter_map(|(index, segment)| {
            if used_indices.contains(&index) {
                None
            } else {
                Some(segment.clone())
            }
        })
        .collect::<Vec<_>>();

    let mut all_rings: Vec<Ring> = vec![];
    for ring_indices in new_rings {
        let ring_segments = ring_indices.into_iter().cloned().collect::<Vec<_>>();
        all_rings.push(Ring::Segments(ring_segments));
    }

    // Circles are rings too
    let circles = sketch
        .primitives()
        .values()
        .filter_map(|s| match s {
            PrimitiveCell::Circle(c) => Some(Ring::Circle(c.borrow().clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    all_rings.extend(circles);

    // Need to implement signed_area
    all_rings.sort_by(|a, b| a.signed_area().total_cmp(&b.signed_area()));

    all_rings = all_rings
        .iter()
        .filter(|r| r.signed_area() > 0.0)
        .cloned()
        .collect();

    (all_rings, unused_segments)
}

pub fn find_next_segment<'seg>(
    segments: impl IntoIterator<Item = &'seg Segment>,
    current_segment: &Segment,
    used_indices: &BTreeSet<usize>,
) -> Option<(usize, &'seg Segment)> {
    let mut matches: Vec<((usize, &Segment), f64)> = vec![];
    let this_segment_end_angle = (current_segment.end_angle() + PI) % (2.0 * PI);

    for (idx, s2) in segments.into_iter().enumerate() {
        if used_indices.contains(&idx) {
            continue;
        }
        if s2.continues(current_segment) && !s2.equals_or_reverse_equals(current_segment) {
            let starting_angle = s2.start_angle();
            let angle_diff = angle_difference(this_segment_end_angle, starting_angle);
            matches.push(((idx, s2), angle_diff));
            // angle_diff measures how hard you'd have to turn left to continue the path from
            // starting_segment to s2, where a straight line would be 180, a left turn 270, a right turn 90.
            // This is important later because to make the smallest loops possible, we always want to be
            // turning left as hard as possible when finding rings.
        }
    }

    matches
        .iter()
        .reduce(|a, b| if a.1 > b.1 { a } else { b })
        .map(|x| x.0)
}

pub fn angle_difference(mut a0: f64, mut a1: f64) -> f64 {
    if a0 > TAU {
        a0 -= TAU;
    }
    if a0 < 0.0 {
        a0 += TAU;
    }

    if a1 > TAU {
        a1 -= TAU;
    }
    if a1 < 0.0 {
        a1 += TAU;
    }

    let mut naive_diff = a1 - a0;
    if naive_diff > TAU {
        naive_diff -= TAU;
    }
    if naive_diff < 0.0 {
        naive_diff += TAU;
    }

    naive_diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::line::Line;
    use crate::primitives::point2::Point2;
    use crate::primitives::PrimitiveCell;
    use geo::{line_string, Coord};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_find_rings_none() {
        let mut sketch = Sketch::new();
        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_b = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(1.0, 1.0)));

        sketch
            .add_primitive(PrimitiveCell::Point2(point_a.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_b.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_c.clone()))
            .unwrap();

        let line_ab = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
        let line_bc = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));

        sketch.add_primitive(PrimitiveCell::Line(line_ab)).unwrap();
        sketch.add_primitive(PrimitiveCell::Line(line_bc)).unwrap();

        let (rings, unused_segments) = find_rings(&sketch);
        assert!(rings.is_empty());
        assert_eq!(unused_segments.len(), 2);
    }

    #[test]
    fn test_find_rings_one() {
        let mut sketch = Sketch::new();
        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_b = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(1.0, 1.0)));

        sketch
            .add_primitive(PrimitiveCell::Point2(point_a.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_b.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_c.clone()))
            .unwrap();

        let line_ab = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
        let line_bc = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
        let line_ca = Rc::new(RefCell::new(Line::new(point_c.clone(), point_a.clone())));

        sketch.add_primitive(PrimitiveCell::Line(line_ab)).unwrap();
        sketch.add_primitive(PrimitiveCell::Line(line_bc)).unwrap();
        sketch.add_primitive(PrimitiveCell::Line(line_ca)).unwrap();

        let (rings, unused_segments) = find_rings(&sketch);
        assert_eq!(rings.len(), 1);
        assert!(unused_segments.is_empty());
    }

    #[test]
    fn test_find_rings_multiple() {
        let mut sketch = Sketch::new();
        let point_a = Rc::new(RefCell::new(Point2::new(-1.0, 0.0)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.0, 1.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));
        let point_d = Rc::new(RefCell::new(Point2::new(0.0, -1.0)));
        let point_e = Rc::new(RefCell::new(Point2::new(2.0, 0.0)));
        let point_f = Rc::new(RefCell::new(Point2::new(3.0, 0.0)));

        for pt in vec![&point_a, &point_b, &point_c, &point_d, &point_e, &point_f] {
            sketch
                .add_primitive(PrimitiveCell::Point2(pt.clone()))
                .unwrap();
        }

        for (start, end) in [
            // Square
            (point_a.clone(), point_b.clone()),
            (point_b.clone(), point_c.clone()),
            (point_c.clone(), point_d.clone()),
            (point_d.clone(), point_a.clone()),
            // First Extension
            (point_b.clone(), point_e.clone()),
            (point_e.clone(), point_d.clone()),
            // Second Extension
            (point_b.clone(), point_f.clone()),
            (point_f.clone(), point_d.clone()),
        ] {
            let line = Rc::new(RefCell::new(Line::new(start, end)));
            sketch.add_primitive(PrimitiveCell::Line(line)).unwrap();
        }

        let (rings, unused_segments) = find_rings(&sketch);
        assert_eq!(rings.len(), 3);
        assert!(unused_segments.is_empty());

        let polys = rings
            .iter()
            .map(|r| r.as_polygon().exterior().clone())
            .collect::<Vec<_>>();
        assert_eq!(
            polys,
            [
                // Extension 1
                line_string![
                    Coord::from((0.0, 1.0)),
                    Coord::from((1.0, 0.0)),
                    Coord::from((0.0, -1.0)),
                    Coord::from((2.0, 0.0)),
                    Coord::from((0.0, 1.0)),
                ],
                // Extension 2
                line_string![
                    Coord::from((0.0, 1.0)),
                    Coord::from((2.0, 0.0)),
                    Coord::from((0.0, -1.0)),
                    Coord::from((3.0, 0.0)),
                    Coord::from((0.0, 1.0)),
                ],
                // Square
                line_string![
                    Coord::from((0.0, 1.0)),
                    Coord::from((-1.0, 0.0)),
                    Coord::from((0.0, -1.0)),
                    Coord::from((1.0, 0.0)),
                    Coord::from((0.0, 1.0)),
                ],
            ],
        );
    }
}
