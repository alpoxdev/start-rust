// 6.2 The match Control Flow Operator

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main2() {
    let coin = Coin::Quarter(UsState::Alabama);
    println!("value_in_cents : {}", value_in_cents(coin));

    let result = plus_one(None);
    println!("result : {:?}", result);

		// The _ Placeholder
		let some_value = 1;
		match some_value {
			1 => println!("One"),
			_ => ()
		}
}
