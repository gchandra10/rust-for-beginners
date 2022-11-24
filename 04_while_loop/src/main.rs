// While Loop

// Body of the loop is processed as long as the condition is True.
fn main() {
    let mut i = 0;
    while i != 6 {
        i += 1;
        println!("inside loop value of i : {i}")
    }
    println!("Exiting the loop once i reaches {}", i);
}
