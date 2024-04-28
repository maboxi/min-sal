use std::cmp::Ordering;
use std::ops::Add;
use std::fmt::Debug;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Distance(pub Option<usize>);

impl Distance {
    pub fn is_inf(&self) -> bool { self.0.is_none() }
    pub fn num_or_inf(&self) -> String {
        match self.0 { None => "∞".to_string(), Some(n) => n.to_string()}}
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num_or_inf())
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0 {
            None => match other.0 {
                None => None,
                Some(_) => Some(Ordering::Greater),
            },
            Some(ref d1) => {
                match other.0 {
                    None => Some(Ordering::Less),
                    Some(ref d2) => d1.partial_cmp(d2)
                }
            }
        }
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        match self.0 {
            None => false,
            Some(ref d1) => {
                match other.0 {
                    None => false,
                    Some(ref d2) => d1.eq(d2)
                }
            }
        }
    }
}

impl Add<Distance> for Distance {
    type Output = Distance;

    fn add(self, other: Self) -> Self::Output {
        match self.0 {
            None => Distance(None),
            Some(d1) => {
                match other.0 {
                    None => Distance(None),
                    Some(d2) => Distance(Some(d1 + d2))
                }
            }
        }
    }
}

impl Add<usize> for Distance {
    type Output = Distance;

    fn add(self, other: usize) -> Self::Output {
        match self.0 {
            None => Distance(None),
            Some(d) => Distance(Some(d + other))
        }
    }
}

impl Add<&usize> for Distance {
    type Output = Distance;

    fn add(self, other: &usize) -> Self::Output {
        match self.0 {
            None => Distance(None),
            Some(d) => Distance(Some(d + other))
        }
    }
}