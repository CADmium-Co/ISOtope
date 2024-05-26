use super::ring::Ring;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Face {
    pub exterior: Ring,
    pub holes: Vec<Ring>,
}

impl Face {
    pub fn from_ring(ring: &Ring) -> Face {
        Face {
            exterior: ring.clone(),
            holes: vec![],
        }
    }

    pub fn add_hole(&mut self, hole: &Face) {
        self.holes.push(hole.exterior.clone());
    }
}
