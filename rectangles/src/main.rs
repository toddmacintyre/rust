fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "Rect 1 {} hold Rect 2",
        match rect1.can_hold(&rect2) {
            true => "can",
            false => "cannot",
        }
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        (other_rectangle.width < self.width && other_rectangle.height < self.height)
            || (other_rectangle.width < self.height && other_rectangle.height < self.width)
    }
}
