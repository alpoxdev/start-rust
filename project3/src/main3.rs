// 3.3 Functions
fn main3() {
    another_function(5, 6);

    // (let y = 6); 는 return value가 없어서 x에 할당하지 못함
    // let x = (let y = 6);

    let x = 5;
    let y = {
        // closure
        let x = 3;
        x + 3
    };

    println!("The y value : {}", y);

    let x = five();
    println!("The five function return value : {}", x);

    let x = plus_one(x);
    println!("The plus_one function return value : {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("another function!");
    println!("parameters are {}, {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
