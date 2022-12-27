/*
Data Type Subsets:
    1. Scalar
    2. Compound

    Scalar:
        Integer:
            8-bit   i8     u8
            16-bit  i16    u16
            32-bit  i32 (Default)   u32
            64-bit  i64    u64
            128-bit i128   u128
            arch    isize  usize
        Floating number:
            f32
            f64 (Default)
        Boolean:
            true
            false
        Char:
            4 bytes
    
    Compound:
        Tuple // fixed in size, collections of different types.
        Array // fixed in size, collections of same type.
*/

fn main() {
    //Numeric Operation
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

}    