use std::{collections::HashMap, hash::Hash};

pub struct Cacher<T>
    where T: Fn(u32) -> u32 {
    pub calculation: T,
    pub result: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32, {
    
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, result: None }
    }

    pub fn execute(&mut self, param: u32) -> u32 {
        match self.result {
            Some(value) => value,
            None => {
                let value = (self.calculation)(param);
                self.result = Some(value);
                value
            }
        }
    }
}

pub struct BetterCacher<T>
    where T: Fn(u32) -> u32 {
    pub calculation: T,
    pub results: HashMap<u32, u32>
}

impl<T> BetterCacher<T>
    where T: Fn(u32) -> u32, {
    
    pub fn new(calculation: T) -> BetterCacher<T> {
        BetterCacher { calculation, results: HashMap::new() }
    }

    pub fn execute(&mut self, param: u32) -> u32 {
        match self.results.get(&param) {
            Some(value) => *value,
            None => {
                let value = (self.calculation)(param);
                self.results.insert(param, value);
                value
            }
        }
    }
}

pub struct SuperCacher<T, K, V>
    where T: Fn(K) -> V {
    pub calculation: T,
    pub results: HashMap<K, V>
}

impl<T, K, V> SuperCacher<T, K, V>
    where T: Fn(K) -> V,
    K: Hash + Eq + Copy,
    V: Clone {
    
    pub fn new(calculation: T) -> SuperCacher<T, K, V> {
        SuperCacher { calculation, results: HashMap::new() }
    }

    pub fn execute(&mut self, param: K) -> V {
        match self.results.get(&param) {
            Some(value) => value.clone(),
            None => {
                let value = (self.calculation)(param);
                self.results.insert(param, value.clone());
                value
            }
        }
    }
}