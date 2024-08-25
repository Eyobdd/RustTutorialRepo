use std::io;
fn main() {
    /* Scalar Types */

    // 1) Integer Types
    /** Length  | Signed | Unsigned
     *  8-bit   | i8     | u8
     *  16-bit  | i16    | u16
     *  32-bit  | i32    | u32
     *  64-bit  | i64    | u64
     *  128-bit | i128   | u128
     *  arch    | isize  | usize
     */
    // 2) Floating Point Types
    let _x = 2.0; //f64 (Default)

    let _y: f32 = 3.0; //f32

    // 3) Numeric Operations

    // a) addition
    let _sum = 5 + 10;

    // b) subtraction
    let _difference = 95.5 - 4.3;

    // c) multiplication
    let _product = 4 * 30;

    // d) division
    let _quotient = 56.7 / 32.2;

    let _truncated = -5 / 3; //Integer division rounded to nearest whole number. Evaluates to -1.

    // e) remainder
    let _remainder = 43 % 5;

    // 4) Boolean Type

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // 5) Character Type

    let _c = 'z';
    let _z = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    /* Compound Types */

    // 1) Tuple Type
    let tup: (i32, f64, u8) = (500, 8.4, 1);

    let (_, y, _) = tup;
    println!("The value of y is: {y}");
    println!("X is: {}, Y is: {}, Z is: {}", tup.0, tup.1, tup.2); // Destructured using index

    // 2) Array Type
    let _arr = [1, 2, 3, 4, 5]; // has fixed size
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // with type annotation
    let _five_3s = [3; 5]; // Array of 5 3s - [3, 3, 3, 3, 3]

    println!("0-index value is: {}, 4-index value is: {}", arr[0], arr[4]);

    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Throws crashes with error if index out of bounds.
    let element = arr[index];

    println!("The value of the element at index {index} is: {element}.");
}
