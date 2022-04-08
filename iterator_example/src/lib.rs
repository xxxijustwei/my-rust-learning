#[derive(Debug, PartialEq)]
pub enum Season {
    SS,
    AW
}

#[derive(Debug, PartialEq)]
pub struct Clothes {
    pub brand: String,
    pub year: u32,
    pub season: Season,
    pub name: String,
    pub price: f32
}

impl Clothes {
    pub fn new(brand: String, year: u32, season: Season, name: String, price: f32) -> Clothes {
        Clothes { brand, year, season, name, price }
    }
}


pub struct Counter {
    count: i32
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}