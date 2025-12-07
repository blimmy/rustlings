// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {

    num * num
    // or it also can be return num * num;
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}


/*
    In Rust, semicolon (;) means “this is a statement, not a value.”
    it can b both way :
        num * num
    // or it also can be return num * num;


 */