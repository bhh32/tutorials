use generics::examples::{custom_generics::*, example_one::example_one};

fn main() {
    // --- Example One Section ---
    // Get the example one data structures and their values
    //let (my_type_vec, str_vec, int_vec) = example_one();

    // Print the values of the data structures
    //println!("{:?}", my_type_vec);
    //println!("{:?}", str_vec);
    //println!("{:?}", int_vec);

    // --- Basic Generic Struct Section ---
    // Create a basic struct with a String
    /*let my_basic_string_struct: MyBasicStruct<String> = MyBasicStruct {
        data: "Basic Generic Struct w/String".into(),
    };

    // Create a basic struct with a u64
    let my_basic_u64_struct: MyBasicStruct<u64> = MyBasicStruct { data: 42 };

    // Create a basic struct with a tuple
    let my_basic_tuple_struct: MyBasicStruct<(f64, i32, &str)> = MyBasicStruct {
        data: (3.14, 42, "Tuple"),
    };

    // Print the values of the basic structs
    println!("{}", my_basic_string_struct.data);
    println!("{}", my_basic_u64_struct.data);
    println!("{:?}", my_basic_tuple_struct.data);*/

    // --- Restricted Type Section ---
    // Create a restricted type with a f64
    let mut my_restricted_type: MyRestrictedType<f64> = MyRestrictedType::new(4.2);
    my_restricted_type.print();
    my_restricted_type.add(2.4);
    my_restricted_type.print();
    my_restricted_type.sub(1.2);
    my_restricted_type.print();

    // Create a restricted type that won't compile
    /*let mut non_compiling_restricted: MyRestrictedType<String> =
        MyRestrictedType::new("I won't compile!".into());
    non_compiling_restricted.print();*/

    // --- Generic Enum Section ---
    // Create a success ApiResponse; NOTICE: The generic type is u32
    let success = ApiResponse::<u32, String>::Success(200);
    // Create an error ApiResponse; NOTICE: The generic type is i32
    let err = ApiResponse::<i32, String>::Error("Not Found".into());

    //let bad_response: ApiResponse<f32, String> = ApiResponse::Error("Bad Response".into());

    match success {
        ApiResponse::Success(code) => println!("Success: {code}"),
        ApiResponse::Error(msg) => println!("Error: {msg}"),
    }

    match err {
        ApiResponse::Success(_) => println!("Success"),
        ApiResponse::Error(msg) => println!("Error: {msg}"),
    }

    // --- Generic Trait Section ---
    // Create a u32 combiner
    let u32_combiner = 42.combine(21);
    println!("{u32_combiner}");

    let string_combiner = "Hello".to_string().combine("Generics!".to_string());
    println!("{string_combiner}");

    // --- Generic Function Section ---
    // Call the generic function with a string
    let _ = repeat_print(&"Hello, Generics!", 3);
    let _ = repeat_print(&42, 2);
}
