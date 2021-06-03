fn main5() {
    // if expression

    let five: i32 = 5;
    if five == 5 {
        println!("The Five!");
    }

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Using if in a let Statement
    // 삼항연산자
    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("number result with condition : {}", number);

    // loop
    loop {
        println!("Hello, World!");
        break;
    }

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 5 {
            break counter + 3;
        }
    };

    println!("the counter {}, the result {}", counter, result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("while | The number is {}", number);
        number -= 1;
    }

    // while with array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < array.len() {
        println!("while | array[{}] : {}", index, array[index]);
        index += 1;
    }

    // for
    for element in array.iter() {
        println!("for | the element : {}", element);
    }

    for number in (1..4).rev() {
        println!("for | the number : {}", number);
    }
}
