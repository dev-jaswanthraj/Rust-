fn main(){
    let mut _tup_one = (); // Empty
    let mut _tup_one = (5.0, 5, 'a');

    const _TUP_TWO: (i32, i8, f32, char) = (10, 1, 1.0, '1');

    //destruction
    let _tup_three = (1, 2, 3, 4);
    let (w, x, y, z) = _tup_three;
    println!("{}-{}-{y}-{}", w, x, z);

    // access element using .
    let tup_four = ('a', 'b');
    let _a: char = tup_four.0;
    let _b: char = tup_four.1;
}

