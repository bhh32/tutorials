pub struct Strings<'a> {
    string_type: String,
    str_type: &'a str,
}

impl<'a> Default for Strings<'a> {
    fn default() -> Self {
        Self {
            string_type: String::from("This is a string! Strings DO NOT implement the copy trait!\nTherefore, they must either be borrowed or cloned if you plan on using the variable again!"),
            str_type: "This is a string slice! They are inherently already borrowed, and therefore\nDO NOT have to be borrowed again or cloned!\nThe important thing to know about the &str type is that it can be used as if it were a vector of chars!",
        }
    }
}

impl<'a> Strings<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_string(&self) -> String {
        self.string_type.clone()
    }

    pub fn get_str_slice(&self) -> &str {
        self.str_type
    }

    // Basic iterator example
    pub fn split_string_on_newline(&self) -> Vec<String> {
        // Get the string_type variable's value
        self.get_string()
            // Split the value on each newline (\n) character
            .split("\n")
            // Split returns &str values, so we have to use map to convert them all to String types.
            // map() takes a closure as its parameter.
            .map(|line| line.to_string())
            // Collect returns the Vec<String> that we will be returning from the function with.
            .collect()
    }

    pub fn get_slice_of_str(&self, start: usize, end: usize) -> &str {
        &self.get_str_slice()[start..end]
    }
}
