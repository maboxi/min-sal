use std::cmp::Ordering;

pub struct Node {
    pub index: usize,
    pub distance: Option<usize>
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.distance {
            None => match other.distance {
                None => None,
                Some(_) => Some(Ordering::Greater),
            },
            Some(ref d1) => {
                match other.distance {
                    None => Some(Ordering::Less),
                    Some(ref d2) => d1.partial_cmp(d2)
                }
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match self.distance {
            None => false,
            Some(ref d1) => {
                match other.distance {
                    None => false,
                    Some(ref d2) => d1.eq(d2)
                }
            }
        }
    }
}