fn main() {
    println!("Simple If ElseIf Else Construct");

    let language = "Rust";

    // if..elseif..else construct

    if language == "Rust" {
        println!("you chose Rust language!");
    } else if language == "Java" {
        println!("You chose Java language!");
    } else {
        println!("None of the above!");
    }

    println!("\n\nNested If");

    let learn_language1 = "Rust";
    let learn_language2 = "Java";

    // Outer if statement
    if learn_language1 == "Rust" {
        // inner if statement
        if learn_language2 == "Java" {
            println!("You are learning Rust and Java language!");
        }
    } else {
        println!("You are learning some other language!");
    }

    println!("\n\nIf as an Expression");

    // If Expression

    //define a variable  
    let learn_language = "Rust";
    // short hand construct
    let res= if learn_language == "Rust" {"You are learning Rust language!"} else {"You are learning some other language!"};
    println!("{}", res);

}
