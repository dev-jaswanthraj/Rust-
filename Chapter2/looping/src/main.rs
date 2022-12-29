fn main() {

    // loop
    let mut count = 0;
    println!("Loop");
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop

    let mut number = 0;

    while number <= 3{
        println!("(while){number}");
        number += 1;
    }

    
    // collection with for 

    let a = [10, 11, 12, 13, 14];

    let mut index = 0;

    while index < 5{
        println!("(while)the value is: {}", a[index]);
        index += 1;
    }

    // for loop
    for element in a{
        println!("(for) {element}");
    }

    // Countdown with for anf range

    for number in (1..4).rev(){
        println!("(for) {number}!");
    }

    println!("LIFTOFF!!!");
}
