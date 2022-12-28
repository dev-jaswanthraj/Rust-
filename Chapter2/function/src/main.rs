fn main() {
    println!("Hello, world!");

    another_function();

    let x = five();
    println!("The value of x is: {x}");

    let y = pluse_one(12);
    println!("The value of y is: {y}");

    another_function_with_parameter(10)
    //print_labeled_measurement(10, 'h');
}

fn another_function(){
    println!("Another function.");
}

fn another_function_with_parameter(x: i32){
    println!("The Value of x i: {x}");
}

// fn print_labeled_measurement(value: i32, unit_label: char){
//     println!("The measurement is: {value} {unit_label}");
// }

fn five() -> i32{
    5
}

fn pluse_one(x: i32) -> i32{
    x + 1
    //return x + 1
}