#[derive(Debug)]
pub struct MyType {
    pub type_msg: String,
}

impl MyType {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            type_msg: msg.into(),
        }
    }
}

pub fn example_one() -> (Vec<MyType>, Vec<String>, Vec<u64>) {
    let my_type_vec: Vec<MyType> = vec![MyType::new("Custom Type!")];
    let str_vec: Vec<String> = vec!["Hello".into(), "Generics!".into()];
    let int_vec: Vec<u64> = vec![1, 2, 3];

    (my_type_vec, str_vec, int_vec)
}
