// 5.1 Defining and Instantiating Structs
use log::{info, trace, warn};

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Using Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main1() {
    // 구조체가 소유권이 없는 데이터의 참조를 저장할수는 있지만,
    // 10장에서 언급 될 라이프타임(lifetimes) 의 사용을 전제로 합니다.
    // 라이프타임은 구조체가 존재하는동안 참조하는 데이터를 계속 존재할 수 있도록 합니다.
    let mut user1 = build_user("contact@alpox.dev".to_string(), "양민열".to_string());
    user1.email = String::from("alpoxdev@gmail.com");

    println!("Hello, world! {}", user1.email);

    let color1 = Color(0, 0, 0);
    let point1 = Point(0, 0, 0);
    println!("{}", color1.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}
