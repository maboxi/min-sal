use std::cmp::{PartialEq, PartialOrd, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub index: usize,
    pub distance: Option<usize>,
    pub predecessor: Option<usize>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        match self.distance {
            None => {
                 match other.distance {
                    None => None,
                    Some(_) => Some(Ordering::Greater),
                 }
            },
            Some(d1) => {
                match other.distance {
                    None => Some(Ordering::Less),
                    Some(d2) => d1.partial_cmp(&d2)
                }
            }
        }
    }
}