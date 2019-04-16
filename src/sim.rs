use std::{marker::PhantomData, default::Default, iter::Iterator};

pub struct Simulation<T: Default> {
    count: u32,
    limit: u32,
    phantom: PhantomData<T>
}

impl<T: Default> Simulation<T> {
    pub fn new(limit: u32) -> Self {
        Self {
            count: 0,
            limit,
            phantom: PhantomData
        }
    }
}

impl<T: Default> Iterator for Simulation<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<T> {
        if self.count < self.limit {
            self.count += 1;
            Some(T::default())
        } else {
            None
        }
    }

}