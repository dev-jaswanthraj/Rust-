fn main() {

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let _x = Some(22);
    let _y = Some("Rust"); 
    let _z = Some(String::from("Rust"));
    let _none: Option<i32> = None;

    //println!("{}", y == z); error

    let x: i8 = 5;
    let y: Option<i8> = None;

    let _sum = x + y.unwrap_or(0);

}