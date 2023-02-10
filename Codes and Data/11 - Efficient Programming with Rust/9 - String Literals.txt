   // -------------------------------------------
   // 			String Literals
   // -------------------------------------------

fn main() {
   
    let str = "The main said \" Hello world \" ";  
    // let str1 = r#"The main said "Hello world""#; 

    //let str = r"The main said _Hello world_ \n \t ' ";   // the simple r works for everything excepts the double quotes 
    
    println!("{}", str); 

    let json_str = "{
        \" name\": \"Micheal\", 
        \"age\": 40,  
        \"sex\": male      
    }";

    let json_str = r#"{
        "name": "Micheal", 
        "age": 44, 
        "sex": male      
    }"#;


    let str = r##"Hello "# World!"##;  // change to r##"Hello "# World!""## later on. 
    println!("{} ", str);  


    // Exercise for you

    let string1 = r#"""#; // " 
    let string2 = r#""""""""#; // """"""
    let string3 = r#" He asked,"Is rust awesome?"""#; // He asked,"Is rust awesome?" 
    println!("{}", string1);
    println!("{}", string2);
    println!("{}", string3);

} 

