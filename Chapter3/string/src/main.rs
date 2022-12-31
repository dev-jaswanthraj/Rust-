fn main() {
    let _s1 = "String Literal";
    let _s2 = String::from("String from string literal"); 
    let mut s3 = String::from("hello");
    s3.push_str(", World!");

    println!("{}", s3)
}
