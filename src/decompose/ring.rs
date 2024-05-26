use super::circle::Circle;
use super::segment::Segment;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Ring {
    Circle(Circle),
    Segments(Vec<Segment>),
}

impl Ring {
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
                if edge_indices_a.len() == 0 {
                    return None;
                } else {
                    Some((edge_indices_a, edge_indices_b))
                }
            }
            _ => None,
        }
    }

    pub fn canonical_form(&self) -> Self {
        // sort the segments in order by first finding the segment with the smallest start point
        // and then rotating the list so that that segment is first
        match self {
            Ring::Circle(circle) => Ring::Circle(circle.clone()),
            Ring::Segments(segments) => {
                let mut canonical_segments: Vec<Segment> = vec![];
                let mut min_index = 0;
                let mut min_segment = segments.get(0).unwrap();
                for (i, segment) in segments.iter().enumerate() {
                    if segment.get_start() < min_segment.get_start() {
                        min_index = i;
                        min_segment = segment;
                    }
                }

                for i in 0..segments.len() {
                    canonical_segments.push(
                        segments
                            .get((i + min_index) % segments.len())
                            .unwrap()
                            .clone(),
                    );
                }

                Ring::Segments(canonical_segments)
            }
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Ring::Circle(circle) => Ring::Circle(circle.clone()),
            Ring::Segments(segments) => {
                let mut reversed_segments: Vec<Segment> = vec![];
                for segment in segments.iter().rev() {
                    reversed_segments.push(segment.reverse());
                }
                Ring::Segments(reversed_segments)
            }
        }
    }
}
