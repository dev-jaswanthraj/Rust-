fn main() {
    let mut s: String = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // error (we are not allowed to have multiple multable refernce to a particular data, but we can have multiple immutable refernces.)
    let mut s1: String = String::from("Rust");

    let r1: &mut String = &mut s1;
    let r2: &mut String = &mut s1; // <-

    println!(" {} {} ", r1, r2);

    // error 2
    let mut s2: String = String::from("Carb");

    let r1: &String = &s2;
    let r2: &String = &s2;

    let r3: &mut String = &s2; // <-

    // error 2 -> solved
    let mut s2: String = String::from("Carb");

    let r1: &String = &s2;
    let r2: &String = &s2;

    println!("{} {}", r1, r2); // from this point on wards the r1 and r2 are outscope.

    let r3: &mut String = &mut s2;
 
    // error 3
    let reference_to_nothing: &String = dangle();
}

fn change(some_string: &mut String){
    some_string.push_str(", World!");
}

// You allowed to have only one mutable refernce to particular piece of data in a particular scope.

fn dangle() -> &String{
    let s: String = String::from("Rust");
    &s
}

fn no_dangle() -> String{
    let s: String = String::from("Rust");

    s
}

