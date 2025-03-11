fn print_int(i: i32) {
    println!("printing copied value of i: {i}");
}

fn print_float(f: f32) {
    println!("printing copied value of f: {f}");
}

fn print_string(s: String) {
    println!("printing moved value of s: {s}");
}

fn print_greeting(copied: &String) {
    println!("copied: {copied}");
}

fn print_greeting_no_copy(owned: String) {
    println!("owned: {owned}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_int() {
        let int = 42;

        // Value is copied NOT moved
        print_int(int);
        // Passes because the value of i is copied
        assert_eq!(int, 42); 

        // Pass the value to a closure
        let double = || int * 2;
        // Passes because the value of i is copied, not moved
        assert_eq!(double(), 84);
        // Also passes
        assert_eq!(int, 42);
    }

    #[test]
    fn test_print_float() {
        let f = 3.14;

        // Value is copied NOT moved
        print_float(f);
        // Passes because the value of f is copied
        assert_eq!(f, 3.14);
    }

    #[test]
    fn test_print_string() {
        let s = String::from("string");

        // Ownership of s is moved into print_string
        // if we don't clone s. So, the below line will
        // work, but we cannot then do the test in the
        // test function.
        //print_string(s);
        //assert_eq!(s, "string".to_string()); // Won't compile

        // So we go ahead and pass s.clone() to we can
        // run the test.
        print_string(s.clone());

        // Passes because the value of s is cloned and
        // the clone is passed into the function, not
        // the original variable.
        assert_eq!(s, "string".to_string());
    }

    #[test]
    fn test_print_greeting() {
        let greeting = String::from("Hi, there!");

        // Value is copied NOT moved
        print_greeting(&greeting);
        print_greeting(&greeting);
        assert_eq!(greeting, "Hi, there!");

        // Because String is NOT a Copy type, when
        // we pass the value in here it is moved!
        //print_greeting_no_copy(greeting);

        // This will not compile and will throw a use after move error
        //assert_eq!(greeting, "Hi, there!");

        // So we go ahead and pass a clone of greeting
        // so we can run the test.
        print_greeting_no_copy(greeting.clone());
        assert_eq!(greeting, "Hi, there!");
    }
}
