fn main() {
    let s1 = "Immutable Array";
    println!("\n{s1:-^30}");

    //define an array of size 4
    let arr: [i32; 4] = [1, 2, 3, 4];

    //print the first element of array
    println!("\nThe first value of array is {}", arr[0]);

    // initialize an array of size 14 with value 10
    let arr1 = [10; 14];

    //print the first element of array
    println!("\nThe first value of array is {}", arr1[0]);

    let s1 = "Mutable Array";
    println!("\n{s1:-^30}");

    //define a mutable array of size 4
    let mut arr2: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("\n{:?}", arr2);
    println!("The value of array at index 1: {}", arr2[1]);

    println!("\n\nAfter changing value of arr2");
    arr2[1] = 9;
    println!("\n{:?}", arr2);
    println!("The value of array at index 1: {}", arr2[1]);

    let s1 = "Get Array Length";
    println!("\n{s1:-^30}");

    println!("\nLength of arr2 is {}", arr2.len());

    let s1 = "Array Slice";
    println!("\n{s1:-^30}");

    //define the slice
    let slice_array1 = &arr2;
    let slice_array2 = &arr2[0..4];
    let slice_array3 = &arr2[4..];
    
    // print the slice of an array
    println!("\nValue of slice_array1: {:?}", slice_array1);
    println!("Value of slice_array2: {:?}", slice_array2);
    println!("Value of slice_array3: {:?}", slice_array3);

}


