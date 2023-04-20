// Enter your code here. Read input from STDIN. Print output to STDOUT
use std::io;
use std::env;

fn main(){

    //env::set_var("RUST_BACKTRACE", "full");
    let mut n =String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read N");
    
    let n: u32 = n.trim().parse().expect("Please enter number");
    
    for _ in 0..n{
        let mut line2 = String::new();
    
        io::stdin().read_line(&mut line2).expect("Failed to read line 2");
        
        // Sprit whitespace will return iter
        let mut line2 = line2.split_whitespace();

        let mut power = line2.next();
        let mut k = line2.next();
        
        let mut power = match power{
            Some(x) => x,
            None => "Not set",
        };
        let mut k = match k{
            Some(x) => x,
            None => "Not set",
        };
        
        let mut power: u32 = power.trim().parse().expect("Error");
        let mut k: u32 = k.trim().parse().expect("Error");
        
        for _ in 0..k-1{
            let mut s = String::new();
            let mut result: u64 = 0;
            io::stdin().read_line(&mut s).expect("Failed to read List of string");
            
            let mut s = s.split_whitespace();
            
            for i in s{
                for j in i.chars(){
                    let mut p: u32 = j as u32;
                    let mut up: u128 = 2_u128.pow(p) - 1;
                    println!("{}", up);
                }
            }
            
            
        }
    }
}
/* 
2
50 3
aceace ceceaa abdbdbdbakjkljhkjh
47 3
azbde abcher acegk

1
6 3
austin dallas houston

 */