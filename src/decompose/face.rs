use serde::{Deserialize, Serialize};

use super::ring::Ring;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
