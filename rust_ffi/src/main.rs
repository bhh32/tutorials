use ffi_example::*;

fn main() {
    // Create the reusable result variable
    let mut result: i32;

    // Call the C sum function
    unsafe {
        result = sum(10, 10);
    }

    println!("c: 10 + 10 = {result}");

    // Call the C diff function
    unsafe {
        result = diff(19, 10);
    }

    println!("c: 19 - 10 = {result}");

    // Call the C quot function
    unsafe {
        result = quot(100, 10);
    }

    println!("c: 100 / 10 = {result}");

    // Call the C prod function
    unsafe {
        result = prod(100, 2);
    }

    println!("c: 100 * 2 = {result}");

    // Call the C++ add function
    unsafe {
        result = add(40, 2);
    }

    println!("c++: 40 + 2 = {result}");

    // Call the C++ sub function
    unsafe {
        result = sub(82, 40);
    }

    println!("c++: 82 - 40 = {result}");

    // Call the C++ mul function
    unsafe {
        result = mul(10, 10);
    }

    println!("c++: 10 * 10 = {result}");

    // Call the C++ div function
    unsafe {
        result = div(1000, 100);
    }

    println!("c++: 1000 / 100 = {result}");
}
