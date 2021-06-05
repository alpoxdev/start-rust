// 4.1 Ownership
// Ownership is Rust’s most unique feature,
// and it enables Rust to make memory safety guarantees
// without needing a garbage collector

fn main1() {
    // What is Ownership?
    // memory is managed through a system of ownership
    // with a set of rules that the compiler checks at compile time

    // 가장 일반적인 데이터 string을 이용해서 ownership을 설명하고자 한다.
    // The stack stores values in the order it gets them and removes the values in the opposite order.
    // 스택은 순서대로 저장되며 순서 역순으로 제거된다. (Last in First out)
    // Data with an unknown size at compile time or a size that might change must be stored on the heap instead
    // 알 수 없는 사이즈, 변경이 되는 사이즈는 힙에 저장된다.
    // 힙에 저장된 데이터에 접근하는 것은 스택에 저장된 데이터에 접근하는 것보다 느린데, 그 이유는 포인터가 가리킨 곳을 따라가야 하기 때문입니다.

    // 소유권 규칙
    // 1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
    // 2. 한번에 딱 하나의 오너만 존재할 수 있다.
    // 3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

    // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
    // 문자열이 변경되지 않는 것을 전재로 하는 특성
    let s = "hello"; // s는 이 지점부터 유효합니다.
                     // 안쓰이면 버려집니다.

    // String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌고,
    // 우리는 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있습니다
    // 메모리를 자동으로 반납함.
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
    println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.

    let s1 = String::from("Hello, World!");
    let s2 = s1;
    // println!("s1 {}, s2 {}", s1, s2);

    // 		let s1 = String::from("Hello, World!");
    //    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    //    |     let s2 = s1;
    //    |              -- value moved here
    //    |     println!("s1 {}, s2 {}", s1, s2);

	let s1 = String::from("Hello, World!");
	let s2 = s1.clone(); // 힙 데이터 복사
	println!("s1 {}, s2 {}", s1, s2);

	let n1 = 5;
	let n2 = n1; // 스택 데이터 복사
	println!("n1 {}, n2 {}", n1, n2);
}
