fn main() {
    let _a = [1, 2, 3, 4, 5]; 
    let _a: [i8; 8] = [0, 1, 2, 3, 4, 5, 6, 7]; // [type; size]

    let a = [13; 10]; // _a = [13, 13, 13, 13, 13, 13, 13, 13, 13, 13]

    let first = a[0];
    let second = a[1];

    println!("{} - {}", first, second);
}
