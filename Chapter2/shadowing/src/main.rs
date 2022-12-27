fn main(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner score is: {x}");
    }
    println!("The value of x is: {x}");

    // another example 
    let space_one = "    ";
    let space_one = space_one.len();

    println!("Length of the space with out mutability is: {space}");
    // warning example
    let mut space_two = "   ";
    let mut space_two = space_two.len();
    println!("Length of the space with mutability is: {space}");
    // error example
    // let mut space_three = "      ";
    // space_three = space_three.len();
    // println!("Length of the space with mutability is: {space}")
}