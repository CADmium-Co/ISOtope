use geo::Polygon;
use serde::{Deserialize, Serialize};

use super::ring::Ring;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Face {
    pub exterior: Ring,
    pub holes: Vec<Face>,
}

impl Face {
    pub fn from_ring(ring: Ring) -> Face {
        Face {
            exterior: ring,
            holes: vec![],
        }
    }

    pub fn add_hole(&mut self, hole: Face) {
        self.holes.push(hole);
    }

    pub fn as_polygon(&self) -> Polygon {
        let exterior = self.exterior.as_polygon();
        let holes = self
            .holes
            .iter()
            .map(|h| h.as_polygon().exterior().clone())
            .collect();
        Polygon::new(exterior.exterior().clone(), holes)
    }
}
