use std::f64::consts::TAU;

use geo::{LineString, Polygon};
use serde::{Deserialize, Serialize};

use crate::primitives::circle::Circle;

use super::segment::Segment;

#[derive(Debug, Clone, PartialOrd, Serialize, Deserialize)]
pub enum Ring {
    Circle(Circle),
    Segments(Vec<Segment>),
}

impl Ring {
    pub fn signed_area(&self) -> f64 {
        match self {
            Ring::Circle(circle) => circle.radius().powi(2) * std::f64::consts::PI,
            Ring::Segments(segments) => {
                let mut area = 0.0;
                for segment in segments {
                    let start = segment.get_start();
                    let end = segment.get_end();
                    area += start.x * end.y - end.x * start.y;
                }
                area / 2.0
            }
        }
    }

    pub fn adjacent_edges(&self, other: &Self) -> Option<(Vec<usize>, Vec<usize>)> {
        match (self, other) {
            (Ring::Segments(segments_a), Ring::Segments(segments_b)) => {
                let mut edge_indices_a: Vec<usize> = vec![];
                let mut edge_indices_b: Vec<usize> = vec![];
                for (index_a, segment_a) in segments_a.iter().enumerate() {
                    for (index_b, segment_b) in segments_b.iter().enumerate() {
                        if segment_a.reverse_equals(segment_b) {
                            edge_indices_a.push(index_a);
                            edge_indices_b.push(index_b);
                        }
                    }
                }
                if edge_indices_a.is_empty() {
                    None
                } else {
                    Some((edge_indices_a, edge_indices_b))
                }
            }
            _ => None,
        }
    }

    pub fn as_polygon(&self) -> Polygon {
        match self {
            Ring::Circle(circle) => {
                let mut b: Vec<(f64, f64)> = vec![];
                let center_ptr = circle.center();
                let center = center_ptr.borrow();

                let num_pts = 36;
                for i in 0..num_pts {
                    let angle = i as f64 / num_pts as f64 * TAU;
                    let x = center.x() + circle.radius() * angle.cos();
                    let y = center.y() + circle.radius() * angle.sin();
                    b.push((x, y));
                }

                Polygon::new(LineString::from(b), vec![])
            }
            Ring::Segments(segments) => {
                // we only ever push the start point. Imagine what happens for a closed
                // square--the final closing segment is inferred.
                // points.push(segments.last().unwrap().get_end());
                let points = segments
                    .iter()
                    .map(|s| {
                        let start = s.get_start();
                        (start.x, start.y)
                    })
                    .collect::<Vec<(f64, f64)>>();

                Polygon::new(LineString::from(points), vec![])
            }
        }
    }
}

impl PartialEq for Ring {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ring::Circle(circle_a), Ring::Circle(circle_b)) => circle_a == circle_b,
            (Ring::Segments(segments_a), Ring::Segments(segments_b)) => {
                segments_a.len() == segments_b.len()
                    && segments_a
                        .iter()
                        .zip(segments_b.iter())
                        .all(|(a, b)| a == b)
            }
            _ => false,
        }
    }
}
