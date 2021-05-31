fn main() {
    // # 3.1 Variables and Mutability

    let x = 5; // immutable constant
    println!("The value of x is {}", x);

    // x = 6; // error because x is immutable constant

    // shadowing - you can declare a new variable with the same name as a previous variable
    // shadowing 은 mut과 다르게 재할당시킬때 사용한다. (constant -> mut, 타입 변환할 때 쓰는 것 같음)
    let mut x = 5; // mutable variable
    x = 6;
    println!("The value of x is {}", x);

    // The other difference between mut and shadowing is that
    // because we’re effectively creating a new variable when we use the let keyword again,
    // we can change the type of the value but reuse the same name
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The spaces {}", spaces);
}
