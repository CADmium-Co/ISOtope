use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use geo::Polygon;
use geo::{Contains as _, InteriorPoint as _};

use crate::{primitives::Primitive, sketch::Sketch};

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

        let random_point_on_face = as_geo_polygon
            .interior_point()
            .expect("Every polygon should be able to yield an interior point");

        let mut located = false;
        for (_old_face_idx, old_face) in old_faces_as_polygons.iter().enumerate() {
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

    if rings.len() == 0 {
        return (faces, unused_segments);
    }

    // this next block of code converts everything to Polygons just so we can
    // determine what faces contain which other faces. It's a bit of a waste
    // because geo is a relatively heavy dependency and we don't need all of it
    let polygons: Vec<Polygon> = rings.iter().map(|r| r.as_polygon()).collect();
    // they are already sorted from smallest to largest area - self.find_rings does this
    let mut what_contains_what: Vec<(usize, usize)> = vec![];

    for smaller_polygon_index in 0..polygons.len() - 1 {
        let smaller_polygon = &polygons[smaller_polygon_index];

        for bigger_polygon_index in smaller_polygon_index + 1..polygons.len() {
            let bigger_polygon = &polygons[bigger_polygon_index];
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
    let init_segments = sketch
        .primitives()
        .iter()
        .map(|p| p.1.borrow().to_primitive())
        .filter_map(|p| match p {
            // We don't consider circles - we'll just add them to the rings directly (right?)
            Primitive::Line(l) => Some(Segment::Line(l)),
            Primitive::Arc(a) => Some(Segment::Arc(a)),
            _ => None,
        })
        .collect::<Vec<Segment>>();
    let init_segments_len = init_segments.len();
    let segments_reversed = init_segments
        .iter()
        .map(|s| s.reverse())
        .collect::<Vec<Segment>>();

    // We consider all given segments and their reversed counterparts
    let all_segments = vec![init_segments, segments_reversed].concat();

    let mut used_indices: Vec<usize> = vec![];
    let mut new_rings: Vec<Vec<usize>> = vec![];

    for (segment_index, segment) in all_segments.iter().enumerate() {
        if used_indices.contains(&segment_index) {
            continue;
        }

        let mut new_ring_indices: Vec<usize> = vec![];
        let start_point = segment.get_start();

        let mut next_segment_index = segment_index;
        for _i in 1..all_segments.len() {
            let next_segment = all_segments.get(next_segment_index).unwrap();
            new_ring_indices.push(next_segment_index);

            next_segment_index = if let Some(index) =
                find_next_segment_index(&all_segments, next_segment, &used_indices)
            {
                index
            } else {
                break;
            };

            if next_segment.get_end() == start_point {
                new_rings.push(new_ring_indices.clone());
                used_indices.extend(new_ring_indices);
                break;
            }
        }
    }

    let used_indices_set = used_indices.into_iter().collect::<BTreeSet<_>>();
    let all_indices_set = (0..all_segments.len()).collect::<BTreeSet<_>>();

    let unused_indices_set = all_indices_set
        .difference(&used_indices_set)
        .collect::<BTreeSet<_>>();
    let unused_indices = unused_indices_set
        .iter()
        .cloned()
        .filter(|index| *index < &init_segments_len)
        .collect::<Vec<_>>();
    let unused_segments = unused_indices
        .iter()
        .map(|index| all_segments.get(**index).unwrap().clone())
        .collect::<Vec<_>>();

    let mut all_rings: Vec<Ring> = vec![];
    for ring_indices in new_rings {
        let ring_segments = ring_indices
            .iter()
            .map(|index| all_segments.get(*index).unwrap().clone())
            .collect::<Vec<_>>();
        all_rings.push(Ring::Segments(ring_segments));
    }

    // Circles are rings too
    let circles = sketch
        .primitives()
        .iter()
        .map(|p| p.1.borrow().to_primitive())
        .filter_map(|s| match s {
            Primitive::Circle(c) => Some(Ring::Circle(c.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    all_rings.extend(circles);

    // Need to implement signed_area
    all_rings.sort_by(|a, b| a.signed_area().partial_cmp(&b.signed_area()).unwrap());

    all_rings = all_rings
        .iter()
        .filter(|r| r.signed_area() > 0.0)
        .cloned()
        .collect();

    (all_rings, unused_segments)
}

pub fn find_next_segment_index(
    segments: &Vec<Segment>,
    current_segment: &Segment,
    used_indices: &Vec<usize>,
) -> Option<usize> {
    let mut matches: Vec<(usize, f64, f64)> = vec![];
    let mut this_segment_end_angle = current_segment.end_angle();
    this_segment_end_angle = (this_segment_end_angle + PI) % (2.0 * PI);

    for (idx, s2) in segments.iter().enumerate() {
        if used_indices.contains(&idx) {
            continue;
        }
        if s2.continues(&current_segment) && !s2.equals_or_reverse_equals(&current_segment) {
            let starting_angle = s2.start_angle();
            let angle_diff = angle_difference(this_segment_end_angle, starting_angle);
            matches.push((idx, starting_angle, angle_diff));
            // angle_diff measures how hard you'd have to turn left to continue the path from
            // starting_segment to s2, where a straight line would be 180, a left turn 270, a right turn 90.
            // This is important later because to make the smallest loops possible, we always want to be
            // turning left as hard as possible when finding rings.
        }
    }

    if matches.len() == 0 {
        None
    } else if matches.len() == 1 {
        Some(matches[0].0)
    } else {
        let mut best_option = 0;
        let mut hardest_left_turn = 0.0;

        for o in matches.iter() {
            if o.2 > hardest_left_turn {
                hardest_left_turn = o.2;
                best_option = o.0;
            }
        }

        Some(best_option)
    }
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
