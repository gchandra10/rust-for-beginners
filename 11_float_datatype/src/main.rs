fn main() {
    
    //implicitly define a float type
    // By default Rust creates a f64 datatype.

    println!("\n\n-------------- Implicit Datatype ----------------");
    let pi = 3.14;
    let e = 2.17828;
    println!("pi: {}", pi);
    println!("e: {}", e);
    

    //explicitly define a float type
    println!("\n\n-------------- Explicit Datatype ----------------");
    let f1:f32 = 32.9;
    let f2:f64 = 6789.89;
    println!("f1: {}", f1);
    println!("f2: {}", f2);
   
    // Declare two float variables with same value

    let my_float1:f64 = 5.0;
    let my_float2:f32 = 5.0;
    
    // Uncomment this next block and build the package.
    // Build will fail because of mismatch in datatypes.

    // if my_float1 as f32 == my_float2 {
    //     println!("both are same");
    // }
    // else {
    //     println!("both are not same");
    // }

    // f32 value is not equal to f64. So converting f64 to f32 works.
    
    println!("\n\n-------------- Datatype Conversion----------------");

    if my_float1 as f32 == my_float2 {
        println!("{my_float1} and {my_float2} are same.");
    }
    else {
        println!("{my_float1} and {my_float2} are not same.");
    }
}

