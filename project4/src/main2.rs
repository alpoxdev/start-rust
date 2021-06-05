// 4.2 References and Borrowing
fn main2() {
    // Reference (참조자)
    let mut string = String::from("Hello, World!");
    let string_size: usize = get_length(&string);
    println!("\"{}\" size is {}", string, string_size);

    // Mutable Reference (가변 참조자)
    change_string(&mut string);
    println!("changed string : \"{}\"", string);

    // 러스트가 컴파일 타임에 데이터 레이스(data race)를 방지
    // error part
    // let r1 = &mut string;
    // let r2 = &mut string;
    // println!("{}, {}", r1, r2);

    // 1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
    // 2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
    // 3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.
    // 우리는 불변 참조자를 가지고 있을 동안에도 역시 가변 참조자를 만들 수 없습니다

    // Dangling Reference
    // 댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안,
    // 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말합니다.

    // dangle();

    // this function's return type contains a borrowed value, but there is no value
    // for it to be borrowed from.
    // (해석: 이 함수의 반환 타입은 빌린 값을 포함하고 있는데, 빌려온 실제 값은 없습니다.)
    let string = no_dangle();
    println!("no_dangle {}", string);
}

// 소유권을 넘기지 않음
// &을 이용해 참조자를 넘김
fn get_length(string: &String) -> usize {
    string.len()
}

fn change_string(string: &mut String) {
    string.push_str(" AlpoxDev");
}

// fn dangle() -> &String {
// 	let string = String::from("Dangling");
// 	&string
// }

fn no_dangle() -> String {
    let string = String::from("no dangle!");
    string
}
