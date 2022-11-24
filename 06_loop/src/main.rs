// Simple Loop


fn main() {

    // Break out of the loop when condition is reached
    println!("Method 1");

    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            break;
        }
    }
    println!("\t Value of x is {x}");
    
    // Break out of the loop with a message.
    println!("Method 2");

    x = 0;
    let v = loop {
        x += 1;
        if x == 5 {
            break "Breaking as x reached 5";
        }
    };
    println!("\t Printing the value of variable v: {}", v);
}

