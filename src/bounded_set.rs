use std::collections::{HashSet, VecDeque};

pub struct BoundedSet<T> {
    capacity: usize,
    set: HashSet<T>,
    order: VecDeque<T>,
}

impl<T: std::hash::Hash + Eq + Clone> BoundedSet<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            set: HashSet::new(),
            order: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.set.contains(&value) {
            return;
        }

        if self.set.len() == self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.set.remove(&old);
            }
        }

        self.order.push_back(value.clone());
        self.set.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
}
