pub mod data;
pub mod helper;

pub mod routeguide {
    tonic::include_proto!("routeguide");
}

use std::hash::{Hash, Hasher};

impl Hash for routeguide::Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.longitude.hash(state);
        self.latitude.hash(state);
    }
}

impl Eq for routeguide::Point {}
