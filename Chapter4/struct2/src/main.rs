struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> (String, bool) {
        if self.width > other.width && self.height > other.height {
            return (String::from("can"), true);
        }
        else{
            return (String::from("can't"), false);
        }
    }
}
impl Rectangle{
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main(){
    let rect1: Rectangle = Rectangle{
        width: 20,
        height: 20,
    };

    let rect2: Rectangle = Rectangle{
        width: 100,
        height: 19,
    };

    println!(
        "Area of the Rect with height: {}, width: {} = {} \nWe {} fit rect with height: {}, width: {} = {}",
        rect1.height,
        rect1.width,
        rect1.area(),
        rect1.can_hold(&rect2).0,
        rect2.height,
        rect2.width,
        rect2.area(),
    );

    let _rect: Rectangle = Rectangle::square(30);
}
