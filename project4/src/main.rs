// 4.2 References and Borrowing
fn main() {
		// Reference (참조자)
    let mut string = String::from("Hello, World!");
    let string_size: usize = get_length(&string);
    println!("\"{}\" size is {}", string, string_size);

		// Mutable Reference (가변 참조자)
    change_string(&mut string);
    println!("changed string : \"{}\"", string);
}

// 소유권을 넘기지 않음
// &을 이용해 참조자를 넘김
fn get_length(string: &String) -> usize {
    string.len()
}

fn change_string(string: &mut String) {
    string.push_str(" AlpoxDev");
}
