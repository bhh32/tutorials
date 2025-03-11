use std::{collections::{
    btree_set, hash_map, hash_set, vec_deque, BTreeSet, HashMap, HashSet, VecDeque
}, fmt::Debug};

// Vec section
pub fn vector_capcity_unspec<T: Clone>(size: usize, default_val: T) -> Vec<T> {
    // Specify the size and the default value for each element
    vec![default_val; size]
}

pub fn vector_capacity_spec<T>(capacity: usize, default_val: T) -> Vec<T> {
    let mut ret_vec: Vec<T> = Vec::with_capacity(capacity);
    ret_vec.push(default_val);

    ret_vec
}

pub fn vector_using_new<T>() -> Vec<T> {
    Vec::new()
}

pub fn vec_pop<T: Debug>(vec: &mut Vec<T>) {
    println!("A Vec can only pop the front element: {vec:#?}");
    vec.pop().unwrap();
    println!("Pop has been called on vec: {vec:#?}");
}

// VecDeque
pub fn vec_deque_pop<T: Debug>(vec_deque: &mut VecDeque<T>) {
    println!("A VecDeque can do a pop on first and last elements - before: {vec_deque:#?}");
    vec_deque.pop_front();
    vec_deque.pop_back();
    println!("Front and back have both been popped: {vec_deque:#?}");
}

pub fn vec_deque_push<T: Debug>(vec_deque: &mut VecDeque<T>, front: T, back: T) {
    println!("A VecDeque can also push to both front and back - before: {vec_deque:#?}");
    vec_deque.push_front(front);
    vec_deque.push_back(back);
    println!("Front and back have both been pushed to: {vec_deque:#?}");
}