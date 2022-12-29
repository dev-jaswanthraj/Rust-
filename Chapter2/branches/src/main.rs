fn main(){
    let number = 30;
    
    // if else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if else if else
    if number % 4 == 0{
        println!("number is divisible by 4");

    } else if number % 3 == 0{
        println!("number is divisible by 3");
    }
    else if number % 2 == 0{
        println!("number is divisible by 2");
    }
    else{
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statment
    let condition = true;
    let number = if condition { 5 } else { 0.1 };

    println!("The value of number is: {number}");
}