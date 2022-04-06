use std::collections::HashMap;

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
    pub result: HashMap<u32, u32>
}

impl<T> BetterCacher<T>
where T: Fn(u32) -> u32, {
    
    pub fn new(calculation: T) -> BetterCacher<T> {
        BetterCacher { calculation, result: HashMap::new() }
    }

    pub fn execute(&mut self, param: u32) -> u32 {
        match self.result.get(&param) {
            Some(value) => *value,
            None => {
                let value = (self.calculation)(param);
                self.result.insert(param, value);
                value
            }
        }
    }
}