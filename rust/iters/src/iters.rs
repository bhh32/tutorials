// Taken from rust by example book
#[derive(Copy, Clone)]
pub struct Fibonacci {
    cur_num: u64,
    next_num: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    // Next is the mandatory function for the Iterator trait
    fn next(&mut self) -> Option<Self::Item> {
        // Custom implementation of what next() does
        let cur = self.cur_num;

        self.cur_num = self.next_num;
        self.next_num = cur + self.next_num;

        Some(cur)
    }
}

// Give the struct some functionality
impl Fibonacci {
    /// Create a new Fibonacci object
    pub fn new() -> Fibonacci {
        Self {
            cur_num: 0,
            next_num: 1,
        }
    }

    /// Print the sequence with an external for loop
    pub fn sequence_with_loop(self, end_num: u64) {
        for num in self.take(end_num as usize) {
            print!("{num} ");
        }
    }

    /// Return the sequence in a Vec<32> just using iterator functionality 
    pub fn sequence_iter_ret_vec(self, end_num: u64) -> Vec<u64> {
        self.take(end_num as usize).map(|out_num| {
            out_num
        }).collect()
    }

    /// Print the sequence directly just using iterator functionality
    pub fn print_sequence(self, end_num: u64) {
        self.take(end_num as usize)
            .map(|out_num| out_num)
            .for_each(|print_num| print!("{print_num} "))
    }
}

pub fn number_iter() {
    // Separate out any previous output.
    println!();

    // Create a range between 1 and, to include, 30 and use the
    // into_iter() function to make it an iterator.
    (1..=30).into_iter()
        // Use the filter() built-in iterator function to test if the element is even.
        // filter() only returns elements that return true
        .filter(|num| num % 2 == 0)
        // Loop through the even elements and print them out on their own line
        // stating them as even!
        .for_each(|num| println!("{num} is even!"));
}