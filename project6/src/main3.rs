#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main3() {
    let some_value1 = Some(1);
    match some_value1 {
        Some(1) => println!("Some Value is 1"),
        None => (),
        _ => (),
    };

    // if let은 '=' 로 구분된 패턴과 표현식을 입력받습니다.
    if let Some(1) = some_value1 {
        println!("Some Value is 1");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(UsState::Alabama) = coin {
        println!("State quarter from {:?}", coin);
    } else {
        count += 1;
    }
}
