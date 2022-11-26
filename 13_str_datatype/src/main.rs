fn main() { 
    // Character Datatype
    // Rust takes up 4 bytes rather than a single byte.

    println!("\n\n-------------- Character Explicit Datatype ----------------");
    let char_2:char = 'a';
    println!("character2: {}", char_2);

    println!("\n\n-------------- Character Implicit Datatype ----------------");
    let char_3 = 'b';
    println!("character3: {}", char_3);


    // String Literal

    println!("\n\n-------------- String Literal Implicit Datatype ----------------");
    // explicitly define 
    let str_1:&str = "Rust Programming";
    println!("String 1: {}", str_1);

    println!("\n\n-------------- String Literal Explicit Datatype ----------------");
    // implicitly define
    let str_2 = "Rust Programming";
    println!("String 2: {}", str_2);

    // String Object

    println!("\n\n-------------- String Object Explicit Datatype ----------------");
    
    let empty_string = String::new();
    println!("Empty String length is {}",empty_string.len());
    
    let content_string = String::from("Rachel Green");
    println!("Content String length is {}",content_string.len());

    // Push Single Character

    println!("\n\n-------------- String Object Push Single Character ----------------");

    let mut name1 = String::from("Hello");
    println!("{}",name1);
    name1.push('!');
    println!("{}",name1);

    // Push a string

    println!("\n\n-------------- String Object Push String ----------------");

    let mut name1 = String::from("Hello");
    println!("{}",name1);
    name1.push_str(" World");
    println!("{}",name1);

    // Convert String Literal to String Object

    println!("\n\n-------------- String Literal to String Object ----------------");

    let name1 = "Hello!".to_string();              //String object
    let name2 = name1.replace("Hello","Howdy");    //find and replace
    println!("{}",name2);


    // Convert String Object to String Literal

    println!("\n\n-------------- String Object to String Literal ----------------");

    let name3 = String::from("hello");
    let name4 = name3.as_str();
    println!("{},{}", name3, name4);
}

