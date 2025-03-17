use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Define a struct that takes a generic type
pub struct MyBasicStruct<T> {
    pub data: T,
}

#[derive(Debug)]
pub struct MyRestrictedType<T>
where
    T: Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy,
{
    pub number: T,
}

impl<T> MyRestrictedType<T>
where
    T: Display + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy,
{
    pub fn new(number: T) -> Self {
        MyRestrictedType { number }
    }

    pub fn add(&mut self, add_value: T) {
        self.number += add_value;
    }

    pub fn sub(&mut self, sub_value: T) {
        self.number -= sub_value;
    }

    pub fn print(&self) {
        println!("{}", self.number);
    }
}

// Generic Enum

// Only takes a integer type!
pub enum ApiResponse<T, String>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + Eq + PartialOrd + Ord + Copy + Display,
{
    Success(T),
    Error(String),
}

// Generic Trait
pub trait Combiner<T> {
    fn combine(&self, item: T) -> T;
}

impl Combiner<u32> for u32 {
    fn combine(&self, item: u32) -> u32 {
        self + item
    }
}

impl Combiner<String> for String {
    fn combine(&self, item: String) -> String {
        format!("{self} {item}")
    }
}

// Generic function
pub fn repeat_print<T: Display + Clone>(item: &T, times: usize) -> Vec<T> {
    let mut ret_items = Vec::new();
    for _ in 0..times {
        println!("{}", item.clone());
        ret_items.push(item.clone());
    }

    ret_items
}
