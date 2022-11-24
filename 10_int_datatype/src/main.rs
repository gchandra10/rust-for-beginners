// see the smallest and biggest numbers,you can use MIN and MAX 

/*
i - signed integer
u - unsigned integer
*/

fn main() {
    // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
    println!("The smallest i8 is {} and the biggest i8 is {}.", i8::MIN, i8::MAX); 
    println!("The smallest u8 is {} and the biggest u8 is {}.", u8::MIN, u8::MAX);
    
    println!("The smallest i16 is {} and the biggest i16 is {}.", i16::MIN, i16::MAX);
    println!("The smallest u16 is {} and the biggest u16 is {}.", u16::MIN, u16::MAX);

    println!("The smallest i32 is {} and the biggest i32 is {}.", i32::MIN, i32::MAX);
    println!("The smallest u32 is {} and the biggest u32 is {}.", u32::MIN, u32::MAX);
    
    println!("The smallest i64 is {} and the biggest i64 is {}.", i64::MIN, i64::MAX);
    println!("The smallest u64 is {} and the biggest u64 is {}.", u64::MIN, u64::MAX);
    
    println!("The smallest i128 is {} and the biggest i128 is {}.", i128::MIN, i128::MAX);
    println!("The smallest u128 is {} and the biggest u128 is {}.", u128::MIN, u128::MAX);

    //This is based on the your OS bit type. 
    println!("The smallest isize is {} and the biggest isize is {}.", isize::MIN, isize::MAX);
    println!("The smallest usize is {} and the biggest usize is {}.", usize::MIN, usize::MAX);

    println!("-----------------------------");

    println!("Implicit Declaration");

    //implicitly define an integer
    let a = 21; 
    let b = 1;
    let c = 54;
    let d = 343434;
    //print the variable
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    println!("-----------------------------");
       
    println!("Explicit Declaration");

    //explicitly define an integer
    let a1:i32 = 24;
    let b1:u64 = 23;
    let c1:u8 = 26;
    let d1:i8 = 29;
    //print the values
    println!("a1: {}", a1);
    println!("b1: {}", b1);
    println!("c1: {}", c1);
    println!("d1: {}", d1);

    println!("-----------------------------");

    println!("Alternate Ways to declare");

    // Alternate Way

    let small_number: u8 = 10;
    
    let small_number1 = 10u8; // 10u8 = 10 of type u8 (no space inbetween 10 and u8)
    
    let big_number = 100000000i32;
    
    let big_number1 = 100_000_000i32; // underscore adds clarity to numbers. It wont affect the value of the number.
 
    let big_number2 = 100_____000________000i32;  //to demonstrate multiple ___
    
    println!("{}\n{}\n{}\n{}\n{}",small_number,small_number1,big_number,big_number1,big_number2);

}


