use std::{io::Error, io::ErrorKind};

#[derive(Clone)]
pub enum Grade {
    Ap,
    A,
    Am,
    Bp,
    B,
    Bm,
    Cp,
    C,
    Cm,
    Dp,
    D,
    Dm,
    F,
    Undefined,
}

#[derive(Clone)]
pub struct Class {
    name: String,
    level: u32,
    instructor: String,
    grade: Grade,
}

impl Class {
    pub fn new(name: impl Into<String>, level: u32, instructor: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level,
            instructor: instructor.into(),
            grade: Grade::A,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn instructor(&self) -> &str {
        &self.instructor
    }

    pub fn grade(&self) -> &str {
        match self.grade {
            Grade::Ap => "A+",
            Grade::A => "A",
            Grade::Am => "A-",
            Grade::Bp => "B+",
            Grade::B => "B",
            Grade::Bm => "B-",
            Grade::Cp => "C+",
            Grade::C => "C",
            Grade::Cm => "C-",
            Grade::Dp => "D+",
            Grade::D => "D",
            Grade::Dm => "D-",
            Grade::F => "F",
            Grade::Undefined => "",
        }
    }

    pub fn update_grade(&mut self, grade: &str) -> Result<(), Error>{
        // Notice the shadowing and the type change
        let grade = match grade.to_uppercase().as_str() {
            "A+" => Grade::Ap,
            "A" => Grade::A,
            "A-" => Grade::Am,
            "B+" => Grade::Bp,
            "B" => Grade::B,
            "B-" => Grade::Bm,
            "C+" => Grade::Cp,
            "C" => Grade::C,
            "C-" => Grade::Cm,
            "D+" => Grade::Dp,
            "D" => Grade::D,
            "D-" => Grade::Dm,
            "F" => Grade::F,
            _ => {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Invalid grade: {grade}; grade not updated...")));
            }
        };

        self.grade = grade;

        Ok(())
    }

    pub fn print_info(&self) {
        println!(
            "{}, Level: {}, Instructor: {}, Grade: {}", 
            self.name(), self.level(), self.instructor(), self.grade()
        );
    }
}

#[derive(Clone)]
pub struct Student {
    fname: String,
    lname: String,
    full_name: String,
    age: u32,
    classes: Vec<Class>
}

impl Student {
    pub fn new(full_name: impl Into<String>, age: u32) -> Result<Self, Error> {
        let mut student = Self {
            fname: String::new(),
            lname: String::new(),
            full_name: full_name.into(),
            age,
            classes: Vec::new()
        };

        let (fname, lname) = match student.full_name.split_once(" ") {
            Some((fname, lname)) => (fname, lname),
            None => {
                eprintln!("Invalid name, cannot create student...");
                return Err(Error::new(ErrorKind::InvalidInput, format!("Invalid name, cannot create student: {}", student.full_name)));
            }
        };

        student.fname = fname.to_string();
        student.lname = lname.to_string();

        Ok(student)
    }

    pub fn get_full_name(&self) -> &str {
        &self.full_name
    }

    pub fn get_fname(&self) -> &str {
        &self.fname
    }

    pub fn get_lname(&self) -> &str {
        &self.lname
    }

    pub fn change_name(&mut self, new_name: impl Into<String>) -> Result<(), Error>{
        let new_name = new_name.into();

        let (fname, lname) = match new_name.split_once(" ") {
            Some((fname, lname)) => (fname, lname),
            None => {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Invalid name, name not updated...")));
            }
        };

        self.fname = fname.into();
        self.lname = lname.into();

        Ok(())
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn update_age(&mut self, age: u32) {
        self.age = age;
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    pub fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }

    pub fn get_classes_mut(&mut self) -> &mut Vec<Class> {
        &mut self.classes
    }

    pub fn print_classes(&self) {
        self.classes.iter().for_each(|class| {
            class.print_info();
        });
    }
}