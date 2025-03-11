use std::collections::vec_deque;

use basics::variables::{collections::{vec_deque_pop, vector_capacity_spec, vector_capcity_unspec}, floating_points::FloatingPoints, integers::Integers, strings::Strings};

fn main() {
    println!("-------------- Integers -------------");
    print_integer_min_max();
    println!();
    println!("------------- Float Points ----------");
    print_floating_min_max();
    println!();
    println!("------------- String Types ----------");
    print_strings();
    println!();
    println!("---------------- Vectors ------------");
    let vec: Vec<Option<u32>> = vector_capcity_unspec(4, None);
    let vec_cap = vector_capacity_spec(42, "The answer!");

    println!("vec no capacity specified - capacity: {}, len: {}", get_cap(&vec), get_len(&vec));
    println!("vec capacity specified - capacity: {}, len: {}", get_cap(&vec_cap), get_len(&vec_cap));
    println!();
    println!("---------------- VecDeque ------------");
    let mut vec_deque = vec_deque::VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_back(2);
    vec_deque.push_back(3);
    vec_deque_pop(&mut vec_deque);
}

fn print_integer_min_max() {
    let integers = Integers::new();
    let (i8_min, i8_max) = integers.get_i8();
    let (i16_min, i16_max) = integers.get_i16();
    let (i32_min, i32_max) = integers.get_i32();
    let (i64_min, i64_max) = integers.get_i64();
    let (i128_min, i128_max) = integers.get_i128();

    println!("For the i8 type, the min, max is: {i8_min}, {i8_max}");
    println!("For the i16 type, the min, max is: {i16_min}, {i16_max}");
    println!("For the i32 type, the min, max is: {i32_min}, {i32_max}");
    println!("For the i64 type, the min, max is: {i64_min}, {i64_max}");
    println!("For the i128 type, the min, max is: {i128_min}, {i128_max}");
}

fn print_floating_min_max() {
    let floating_points = FloatingPoints::new();
    let (f32_min, f32_max) = floating_points.get_f32();
    let (f64_min, f64_max) = floating_points.get_f64();

    println!("For the f32 type, the min, max is: {f32_min}, {f32_max}");
    println!("For the f64 type, the min, max is: {f64_min}, {f64_max}");
}

fn print_strings() {
    let strings = Strings::new();
    let string_type = strings.get_string();
    let str_slice = strings.get_str_slice();

    println!("{string_type}");
    println!();
    println!("{str_slice}");
    println!();
    println!("String slice range 0..23: {}", strings.get_slice_of_str(0, 23));
}

fn get_cap<T>(vec: &Vec<T>) -> usize {
    vec.capacity()
}

fn get_len<T>(vec: &[T]) -> usize {
    vec.len()
}
