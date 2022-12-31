#[derive(Debug)]
struct Rectangle{
    wigth: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.wigth * self.height
    }
}

fn main(){
    let rect: Rectangle = Rectangle{
        wigth: 30,
        height: 40,
    };

    println!("{:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels. (impl of Rectangle)",
        rect.area()
    );

    println!(
        "The area of the rectangle is {} square pixels. (Seperate function)",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.wigth * rectangle.height
}