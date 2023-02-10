   // -------------------------------------------
   // 			Todo macro
   // -------------------------------------------


// Example #1: 
#[derive(Default)]
struct Student{
    name_std: String,
    age: u8, 
    sex: char,
    country: String,
    salary: u32,
    nationality: String,
}


impl Student {
    fn some_fn_1(&self) -> String {    
    todo!()   
    // "".to_string()             
    } 


    fn some_fn_2(&self) -> u8 {
    todo!()
    }
}


fn main() {
    let student_1 = Student::default(); 
    student_1.some_fn_1();

}


trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);     

    fn country_info(&self) -> &str; 
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) { 
        todo!()
    }

    fn country_info(&self) -> &str {
        todo!()
    }   
}

