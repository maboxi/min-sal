use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::fmt::Debug;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Distance<T>(Option<T>);

impl<T: Copy> Distance<T> {
    pub fn is_inf(&self) -> bool { self.0.is_none() }
    pub fn from_ref(n: &T) -> Distance<T> { Distance(Some(*n)) }
    pub fn from(n: T) -> Distance<T> { Distance(Some(n)) }

    pub fn from_other<U, F>(other_dist: &Distance<U>, cast: F) -> Distance<T> 
    where
        F: FnOnce(&U) -> T
    {
        match &other_dist.0 {
            None => Distance(None),
            Some(d) => Distance(Some(cast(d)))
        }
    }

    pub fn inf() -> Distance<T> { Distance(None) }
    pub fn value(&self) -> T { self.0.unwrap() }
}

impl<T: fmt::Display> Distance<T> {
    pub fn num_or_inf(&self) -> String {
        match &self.0 { None => "∞".to_string(), Some(n) => n.to_string()}}
}

impl<T: fmt::Display> fmt::Display for Distance<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num_or_inf())
    }
}

impl<T: PartialOrd> PartialOrd for Distance<T> {
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

impl<T: PartialEq> PartialEq for Distance<T> {
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

impl<T: Add<T, Output = T>> Add<Distance<T>> for Distance<T> {
    type Output = Distance<T>;

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

impl<T: Add<T, Output = T>> Add<T> for Distance<T> {
    type Output = Distance<T>;

    fn add(self, other: T) -> Self::Output {
        match self.0 {
            None => Distance(None),
            Some(d) => Distance(Some(d + other))
        }
    }
}

impl<T: Sub<T, Output=T>> Sub<T> for Distance<T> {
    type Output = Distance<T>;

    fn sub(self, other: T) -> Self::Output {
        match self.0 {
            None => Distance(None),
            Some(d) => Distance(Some(d - other))
        }
    }
}