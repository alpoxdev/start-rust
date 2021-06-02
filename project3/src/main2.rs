use std::io;

// 3.2 Data Types
fn main2() {
    // integer
    // signed :  -(2^(n - 1)) to 2^(n - 1)
    // unsigned : 0 to 2^n - 1
    let bit8_signed: i8 = -1;
    let bit8_unsigned: u8 = 1;
    let bit16_signed: i16 = -1;
    let bit16_unsigned: u16 = 1;
    let bit32_signed: i32 = -1;
    let bit32_unsigned: u32 = 1;
    let bit64_signed: i64 = -1;
    let bit64_unsigned: u64 = 1;
    let bit128_signed: i64 = -1;
    let bit128_unsigned: u64 = 1;

    // computer architecture
    // 64bit computer & 32bit computer
    // commonly 64bit
    let size_signed: isize = -1;
    let size_unsigned: usize = 1;

    // _ as a visual separator, such as 1_000
    println!("{}", 100_100);

    // if ur unsure, Rust's defaults are generally good choices
    // integer types default to "i32" : this type is generally the fastest, even on 64-bit systems
    let default_integer: i32 = 13;
    println!("default_integer: {}", default_integer);

    // integer overflow
    // when ur compiling in debug mode,
    // Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs
    // “Unrecoverable Errors with panic!”
    // in --release flag, Rust does not include checks for integer overflow

    // float
    // f32, f64
    // default float type to f64
    let float_64 = 1.0;
    let float_32: f32 = 2.0;

    // boolean
    let boolean_type: bool = false;

    // character
    let character = 'c';

    // compound types

    // Tuple Type
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Array Type
    let array: [i32; 3] = [1, 2, 3];
    println!("[{}, {}, {}]", array[0], array[1], array[2]);

    let fill_array = [3; 5];

    // Invalid Array Element Access
    // If the index is greater than or equal to the length, Rust will panic.
    let example_array = [1, 2, 3, 4, 5];
    println!("Please enter an array index!");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fail to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = example_array[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}
