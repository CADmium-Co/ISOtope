use super::ring::Ring;

#[derive(Debug)]
pub struct Face {
    pub exterior: Ring,
    pub holes: Vec<Ring>,
}

impl Face {
    pub fn from_ring(ring: Ring) -> Face {
        Face {
            exterior: ring,
            holes: vec![],
        }
    }

    pub fn add_hole(&mut self, hole: Ring) {
        self.holes.push(hole);
    }
}
