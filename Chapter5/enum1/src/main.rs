// Defining Enum 
enum IpAddrKindOne{
    V4,
    V6
}

// Values inside Enum
enum IpAddrKindTwo{
    V4(String),
    V6(String),
}
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Messge{
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Enum Method
impl Messge{
    fn some_function() {
        println!("Let's get Rusty!");
    }
}

// Enums in Strunct
struct IpAddr{
    kind: IpAddrKindOne,
    address: String,
}

fn main() {

    let _four: IpAddrKindOne = IpAddrKindOne::V4;
    let _six: IpAddrKindOne = IpAddrKindOne::V6;

    let _localhost_8080: IpAddr = IpAddr{
        kind: IpAddrKindOne::V4,
        address: String::from("127.0.0.1:8080"),
    };

    let _localhost_5000: IpAddrKindTwo = IpAddrKindTwo::V4(String::from("127.0.0.1:5000"));

    let _localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    
}