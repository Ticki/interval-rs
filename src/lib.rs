#![feature(box_patterns)]
#![feature(box_syntax)]

#[derive(Clone, PartialEq)]
pub enum Set<T> 
         where T: PartialOrd + PartialEq + Clone {
    Greater(T),
    Equal(T),
    Union(Box<Set<T>>, Box<Set<T>>),
    Not(Box<Set<T>>),
    All,
}

impl<T> Set<T>
        where T: PartialOrd + PartialEq + Clone {
    pub fn intersection(self, other: Self) -> Self {
        Set::Not(box Set::Union(box Set::Not(box self), box Set::Not(box other)))
    }
    pub fn union(self, other: Self) -> Self {
        Set::Union(box self, box other)
    }
    pub fn not(self) -> Self {
        Set::Not(box self)
    }
    pub fn contains(self, element: T) -> bool {
        match self {
            Set::Greater(than) => element > than,
            Set::Equal(to) => element == to,
            Set::Union(box a, box b) => a.contains(element.clone()) || b.contains(element),
            Set::Not(box x) => !x.contains(element),
            Set::All => true,
        }
    }
    pub fn is_empty(self) -> bool {
        self.not().is_universe()
    }
    pub fn is_universe(self) -> bool {
        match self {
            Set::All => true,
            Set::Union(box a, box b) => a.is_universe() || b.is_universe(),
            Set::Not(box x) => !x.is_universe(),
            _ => false,
        }
    }
    pub fn is_subset(self, other: Self) -> bool {
        other.not().intersection(self).is_empty()
    }
}
