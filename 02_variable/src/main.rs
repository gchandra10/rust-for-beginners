fn main() {
    // Create an immutable Scalar variable with String Literal. 
    let name = "Rachel";
    // By default all variables are immutable
    let age = 30;

    // {} are called Placeholders
    println!("Method 1 - Hello {},{}", name, age);
    // Alternate way to print
    println!("Method 2 - Hello {name},{age}");
}
