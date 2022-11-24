// For Loops

fn main() {

    // Left side Inclusive, Right side exclusive
    println!("Left side Inclusive and Right side value exclusive. So it prints upto 4.");

    for x in 0..5 {
        println!("{}", x);
    }

    // By adding = both sides are inclusive

    println!("Both Left and Right sides are inclusive. So it prints upto 5.");

    for x in 0..=5 {
        println!("{}", x);
    }
}
