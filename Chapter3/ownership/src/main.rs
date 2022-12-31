fn main() {
    a();
    fn a(){
        let _x: &str = "hello"; // immutable
        let _y: i32 = 22;

        b();
    }

    fn b() {
        let _x: String = String::from("Rust"); // mutable
    }


    { // s is not valid here, i's not yet declared
        let _s: String = String::from("Ownership"); // s is vaild from this point forward onwards
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    {
        let x: i32 = 5;
        let _y: i32 = x;  // Copy

        let s1: String = String::from("Rust");
        let _s2: String = s1.clone();

        println!("{}, world!", s1);
    }

    // borrowing
    let s: String = String::from("Rust!");
    print_out(&s);
    println!("{}", s);

    fn print_out(str: &String){
        println!("{} in print_out function", str);
    }
}



