// 5.3 Method Syntax

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn set_width(&mut self, width: usize) {
        self.width = width;
    }
    fn set_height(&mut self, height: usize) {
        self.height = height;
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 32,
        height: 32,
    };

    let rect2 = Rectangle {
        width: 24,
        height: 24,
    };

    println!("{:#?},\nrect area : {}", rect1, rect1.area());
    println!("can_hold result : {}", rect1.can_hold(&rect2));

    rect1.set_width(24);
    println!("{:#?}\n rect area : {}", rect1, rect1.area());
    println!("can_hold result : {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(16);
    println!("The Square : {:#?}", square);
}
