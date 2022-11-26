
fn main() {

    // concatenation

    println!("\n\n-------------- String Concatenation ----------------");

    let str1 = "Hello".to_string();
    let str2 = " world".to_string();
    let str3 = str1 + &str2;
    println!("{}", str3);
  
    // Reverse String

    println!("\n\n-------------- String Reverse ----------------");

    let s = "Hello World";
    let t: String = s.chars().rev().collect();
    println!("{} - {}", s, t);

    // Palindrome

    println!("\n\n-------------- Palindrome ----------------");

    let s = "rotator";
    let t: String = s.chars().rev().collect();
    
    if s == t {
        println!("{s} is Palindrome")
    }
    else{
        println!("{s} is Not Palindrome")
    }

    // String Padding

    println!("\n\n-------------- String Padding ----------------");
    
    println!("\n");

    let s1 = "pizza";
    
    println!("{s1:-^30}");
    println!("{s1:-<30}");
    println!("{s1:->30}");

}

