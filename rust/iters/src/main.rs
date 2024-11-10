mod iters;

use std::thread;
use futures::{join, executor::block_on};
use iters::*;

fn main() {
    block_on(async_main());
}

async fn async_main() {
    // Fibonacci Sequence using a custom iterator
    let fib = Fibonacci::new();

    let seq_loop = run_seq_loop(50, fib);
    let seq_vec_loop = run_seq_vec_loop(20, fib);
    let seq_iter = run_seq_iter(80, fib);

    join!(seq_loop, seq_vec_loop, seq_iter);

    number_iter();
}

fn print_msg(digits: u8, new_lines: Option<u8>) {
    match new_lines {
        Some(2) => println!("\n\nPrinting the fibonacci sequence up to {digits} digits:"),
        Some(_) | None => println!("Printing the fibonacci sequence up to {digits} digits"),
    }
}

async fn run_seq_loop(iterations: u8, fib: Fibonacci) {
    // Print the sequence using the sequence_with_loop() function
    thread::spawn(move || {
        print_msg(iterations, None);
        fib.clone().sequence_with_loop(iterations as u64);
    })
    .join()
    .unwrap();
}

async fn run_seq_vec_loop(iterations: u8, fib: Fibonacci) {
    // Print the sequence using the sequence_iter_ret_vec() function
    thread::spawn(move || {
        print_msg(iterations, Some(2));
        let sequence = fib.clone().sequence_iter_ret_vec(iterations as u64);
        // Notice to print the sequence we have to iterate through the returned vec
        for num in sequence.iter() {
            print!("{num} ");
        }
        println!();
    })
    .join()
    .unwrap();
}

async fn run_seq_iter(iterations: u8, fib: Fibonacci) {
    // Print the sequence with the print_sequence() function.
    thread::spawn(move || {
        print_msg(iterations, Some(2));
        fib.clone().print_sequence(iterations as u64);
        println!();
    })
    .join()
    .unwrap();
}
