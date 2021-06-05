#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main2(){
	let rect1 = Rectangle { width: 32, height: 32};
	println!("{:?}, {:#?}, {}", rect1, rect1, area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}