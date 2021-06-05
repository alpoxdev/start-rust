// 4.3 The Slice Type
fn main3() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);
    s.clear();

    // 스트링 슬라이스는 String의 일부분에 대한 참조자고, 아래와 같이 생겼습니다:
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("word : {}", word);

		// 불변 참조자 &str
		let _s = "Hello, world!";
		// fn first_word(s: &str) -> &str {
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
