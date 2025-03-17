pub mod examples;
use examples::{custom_generics::*, example_one::*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        // Get the example one data structures and their values
        let (my_type_vec, str_vec, int_vec) = example_one();

        // Test the values of the data structures
        assert_eq!(my_type_vec[0].type_msg, "Custom Type!");
        assert_eq!(str_vec[0], "Hello");
        assert_eq!(str_vec[1], "Generics!");
        assert_eq!(int_vec[0], 1);
        assert_eq!(int_vec[1], 2);
        assert_eq!(int_vec[2], 3);
    }

    #[test]
    fn test_basic_struct() {
        // Create a basic struct with a string
        let my_basic_string_struct: MyBasicStruct<String> = MyBasicStruct {
            data: "Basic Generic Struct".into(),
        };

        // Create a basic struct with a u64
        let my_basic_u64_struct: MyBasicStruct<u64> = MyBasicStruct { data: 42 };

        // Create a basic struct with a tuple
        let my_basic_tuple_struct: MyBasicStruct<(f64, i32, &str)> = MyBasicStruct {
            data: (3.14, 42, "Tuple"),
        };

        // Test the values of the basic structs
        assert_eq!(
            my_basic_string_struct.data,
            "Basic Generic Struct".to_string()
        );
        assert_eq!(my_basic_u64_struct.data, 42);
        assert_eq!(my_basic_tuple_struct.data, (3.14, 42, "Tuple"));
    }

    #[test]
    fn test_restricted_type() {
        // Create a restricted type with a u64
        let mut my_restricted_type: MyRestrictedType<u64> = MyRestrictedType::new(42);

        // Test the value of the restricted type
        assert_eq!(my_restricted_type.number, 42);

        // Add 10 to the restricted type
        my_restricted_type.add(10);

        // Test the value of the restricted type
        assert_eq!(my_restricted_type.number, 52);

        // Subtract 5 from the restricted type
        my_restricted_type.sub(5);

        // Test the value of the restricted type
        assert_eq!(my_restricted_type.number, 47);
    }

    #[test]
    fn test_generic_enum() {
        // Create a success ApiResponse; NOTICE: The generic type is u32
        let success = ApiResponse::<u32, String>::Success(200);

        // Create an error ApiResponse; NOTICE: The generic type is i32
        let err = ApiResponse::<i32, String>::Error("Not Found".into());

        // Test the success ApiResponse
        match success {
            ApiResponse::Success(code) => assert_eq!(code, 200),
            ApiResponse::Error(_) => panic!("Expected Success"),
        }

        // Test the error ApiResponse
        match err {
            ApiResponse::Success(_) => panic!("Expected Error"),
            ApiResponse::Error(msg) => assert_eq!(msg, "Not Found"),
        }
    }

    #[test]
    fn test_generic_trait() {
        // Create a u32 combiner
        let u32_combiner = 42.combine(24);

        // Test the u32 combiner
        assert_eq!(u32_combiner, 66);
    }

    #[test]
    fn test_generic_trait_string() {
        // Create a string combiner
        let string_combiner = "Hello".to_string().combine("World".to_string());

        // Test the string combiner
        assert_eq!(string_combiner, "Hello World");
    }

    #[test]
    fn test_generic_function() {
        let rep_print_str = repeat_print(&"Hello, Generics!", 3);
        let rep_print_int = repeat_print(&42, 2);

        let rep_str_result: String =
            rep_print_str
                .iter()
                .fold(String::new(), move |msg, next_msg| {
                    let str_res = format!("{msg}{next_msg}");
                    str_res
                });

        let rep_print_int: String = rep_print_int.iter().fold(String::new(), |num, next_num| {
            let str_res = format!("{num}{next_num}");
            str_res
        });

        assert_eq!(
            &rep_str_result,
            "Hello, Generics!Hello, Generics!Hello, Generics!"
        );
        assert_eq!(&rep_print_int, "4242");
    }
}
