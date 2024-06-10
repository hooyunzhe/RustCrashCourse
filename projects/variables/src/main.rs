use std::io;

fn main() {
    // Variables and Mutability

    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");


    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is {x}");


    // let spaces = "   ";
    // let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len();



    // Data Types
    // let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!");


    // Integer Types
    // let signed_8bit: i8 = -42;
    // let unsigned_8bit: u8 = 42;
    // let signed_16bit: i16 = -4242;
    // let unsigned_16bit: u16 = 4242;
    // let signed_32bit: i32 = -42424242;
    // let unsigned_32bit: u32 = 42424242;
    // let signed_64bit: i64 = -424242424242424242;
    // let unsigned_64bit: u64 = 424242424242424242;
    // let signed_128bit: i128 = -42424242424242424242424242424242424242;
    // let unsigned_128bit: u128 = 42424242424242424242424242424242424242;
    // let signed_arch: isize = -424242424242424242;
    // let unsigned_arch: usize = 424242424242424242;

    // println!("Signed (8 bit): {signed_8bit}");
    // println!("Unsigned (8 bit): {unsigned_8bit}");
    // println!("Signed (16 bit): {signed_16bit}");
    // println!("Unsigned (16 bit): {unsigned_16bit}");
    // println!("Signed (32 bit): {signed_32bit}");
    // println!("Unsigned (32 bit): {unsigned_32bit}");
    // println!("Signed (64 bit): {signed_64bit}");
    // println!("Unsigned (64 bit): {unsigned_64bit}");
    // println!("Signed (128 bit): {signed_128bit}");
    // println!("Unsigned (128 bit): {unsigned_128bit}");
    // println!("Signed (arch): {signed_arch}");
    // println!("Unsigned (arch): {unsigned_arch}");


    // Floating-Point Types
    // let x = 2.0; // f64 by default due to its superior precision
    //                   // while having roughly the same speed

    // let y: f32 = 3.0;

    // println!("Float (64 bit): {x}");
    // println!("Float (32 bit): {y}");


    // Numeric Operations
    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;

    // // remainder
    // let remainder = 43 % 5;

    // println!("5 + 10 = {sum}");
    // println!("95.5 - 4.3 = {difference}");
    // println!("4 * 30 = {product}");
    // println!("56.7 / 32.2 = {quotient}");
    // println!("-5 / 3 = {truncated}");
    // println!("43 % 5 = {remainder}");


    // The Boolean Type
    // let t = true;
    // let f: bool = false;
    // println!("Boolean (positive): {t}");
    // println!("Boolean (negative): {f}");


    // The Character Type
    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';

    // println!("ASCII character: {c}");
    // println!("Unicode character: {z}");
    // println!("Emoji character: {heart_eyed_cat}");


    // The Tuple Type
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // let a = tup.0;
    // let b = tup.1;
    // let c = tup.2;

    // println!("Tuple (destructured): ({x}, {y}, {z})");
    // println!("Tuple (accessed with index):");
    // println!("                    tuple.0: {a}");
    // println!("                    tuple.1: {b}");
    // println!("                    tuple.2: {c}");


    // The Array Type
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a0 = a[0];
    // let a1 = a[1];
    // let a2 = a[2];
    // let a3 = a[3];
    // let a4 = a[4];
    // let dups = [42; 5];
    // let d0 = dups[0];
    // let d1 = dups[1];
    // let d2 = dups[2];
    // let d3 = dups[3];
    // let d4 = dups[4];

    // println!("Array: [{a0}, {a1}, {a2}, {a3}, {a4}]");
    // println!("Array (same values): [{d0}, {d1}, {d2}, {d3}, {d4}]");

    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index  = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
}
