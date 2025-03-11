use shadow::examples::grade_example::*;
use std::collections::HashMap;

fn main() {
    // Create a HashMap to store students
    let mut students: HashMap<&str, Student> = HashMap::new();

    // Add Bryan Hyland to the HashMap
    students.insert("Bryan Hyland", create_student("Bryan Hyland", 39));

    // Add classes to Bryan
    match students.get_mut("Bryan Hyland") {
        Some(student) => {
            student.add_class(Class::new("Rust Lang", 401, "Rey Skywalker"));
            student.add_class(Class::new("Go", 101, "Clark Kent"));
            student.add_class(Class::new("Python", 201, "Diana Prince"));
            student.add_class(Class::new("Embedded Rust", 102, "Victor Freeze"));
        },
        None => {
            eprintln!("Error: Student name given not found in students...");
            return;
        }
    }

    // Print the information
    for student in students.values() {
        println!("{} {} Number of Classes: {}", student.get_full_name(), student.get_age(), student.get_classes().len());
        println!("Class List:");
        for class in student.get_classes() {
            class.print_info();
        }
    }

    println!("\n"); // Gain some space for readability

    // Bryan did extra credit that bumped his grade in Rust Lang to an A+
    if let Some(bryan) = students.get_mut("Bryan Hyland") {
        let _: Vec<_> = bryan.get_classes_mut()
            .iter_mut()
            .filter(|class| class.name() == "Rust Lang")
            .map(|class| {
                // Make sure we do our proper error handling!
                match class.update_grade("a+") {
                    Ok(_) => println!("Updated grade successfully!"),
                    Err(e) => eprintln!("Error updating grade: {e}"),
                };
            })
            .collect();
    } else {
        eprintln!("Error: Student name given not found in students");
    }

    // Gain some more space for the updated information
    println!("\n");

    // Print the updated information for the `Rust Lang` class
    // ***NOTICE***: We are not actually shadowing here even though it looks like
    // we are! 
    // The previous `if let Some(bryan)` variable is no longer in scope, and
    // therefore we are not actually shadowing the variable!
    let bryan = match students.get("Bryan Hyland") {
        Some(student) => student.clone(),
        None => {
            eprintln!("Error: Student name given not found in students...");
            return;
        }
    };
    
    // Print a "Student Header"
    println!("{}\t{}\tNumber of Classes: {}", bryan.get_full_name(), bryan.get_age(), bryan.get_classes().len());
    
    // Print the updated information for the `Rust Lang` class
    let _: Vec<_> = bryan.get_classes()
        .iter()
        .filter(|class| class.name() == "Rust Lang")
        .map(|class| class.print_info())
        .collect();   
    

}

// Helper functions
fn create_student(full_name: &str, age: u32) -> Student {
    match Student::new(full_name, age) {
        Ok(student) => student,
        Err(e) => {
            eprintln!("Error creating student: {}", e);
            panic!("Failed to create student");
        }
    }
}