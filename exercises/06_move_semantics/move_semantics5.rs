#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {

    data.chars().last().unwrap()

}


// Should take ownership
fn string_uppercase(mut data: String) {

    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();
    get_char(&data);
    string_uppercase(data);
}


/*
    get_char — Shouldn't take ownership
    bc im just looking for the char dont change
    anything

    string_uppercase — Should take ownership
    bc we use this func to change it in uppercase

    how to get ownership - fill e & in front of the type
    and like this get_char(&data); when u wanna call

 */