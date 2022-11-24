fn main() {
    // Create an mutable Scalar variable with String Literal. 
    let mut name = "Rachel";
    let mut age = 30;

    // {} are called Placeholders

    println!("Method 1 - Hello {},{}", name, age);

    // Changing the values

    name = "Rachel Green";
    age = 29;

    println!("Method 2 - Hello {name},{age}");
}
