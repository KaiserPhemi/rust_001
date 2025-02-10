use std::string;

fn main() {
    println!("Hello, Universe");

    // primitive data types

    // interger
    let x: i32 = -56;
    let y: u64 = 345;

    println!("Signed Int: {}", x);
    println!("Unsigned Int: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Signed Int: {}", e);
    println!("Unsigned Int: {}", i);

    // float types
    let pi: f64 = 3.14;
    println!("Print float: {}", pi);

    // boolean
    let is_married: bool = false;
    println!("Checking status : {}", is_married);

    // compound data types
    // arrays, tuples, slices strings

    let numbers_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers array: {:?}", numbers_arr);

    let food_arr: [&str; 3] = ["eba", "ila", "rice"];
    println!("My foods: {:?}", food_arr);
    println!("My foods: {}", food_arr[0]);

    // Tuple
    let sapien: (String, i32, bool) = ("FEMI".to_string(), 45, false);
    print!("My data:{:?} ", sapien);

    let my_mix_data: (String, i32, bool) = ("Akinwa".to_string(), 45, true);
    println!("The data: {:?}", my_mix_data);

    // slices: [1,2,3,4,5]
    let num_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("The Slice: {:?}", num_slice);

    let book_slices: &[&str] = &["Hasrry Porter", "Alita", "Game of Throne"];
    println!("The books: {:?}", book_slices);

    // Strings vs string slices
    // A-Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Heaven, ");
    stone_cold.push_str("Yeah");
    println!("Stone Cold : {}", stone_cold);

    // B-&str (string slice) immutable
    let b_string: String = String::from("Hello Universe");
    let slicer: &str = &string;
    println!("Slive value:  {}", slicer)
}
