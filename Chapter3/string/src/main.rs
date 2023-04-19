fn main() {
    let _s1 = "String Literal";
    let _s2 = String::from("String from string literal"); 
    let mut s3 = String::from("hello");
    s3.push_str(", World!");

    println!("{}", s3);

    // Example
    let j = String::from("Jaswanthraj R");
    let r = String::from("Rust");

    let j_r = j + &r;

    println!("{}",j_r);

    let str1 = String::from("YA");
    let str2 = String::from("AA");
    let str3 = String::from("WN");

    let _concat_str = format!("{}-{}-{}", str1, str2, str3);

    for char_demo in "こんにちは".chars() {
        println!("{}", char_demo);
    }
}
