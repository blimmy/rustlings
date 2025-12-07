// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}


/*
+--------------------------------------+----------------+----------------------------------------------+
| Situation                             | to_string()?  | Example                                      |
+--------------------------------------+----------------+----------------------------------------------+
| Function returns String              | YES ✅         | fn color() -> String {                       |
|                                      |                |     "blue".to_string()                       |
|                                      |                | }                                            |
+--------------------------------------+----------------+----------------------------------------------+
| Struct field is String               | YES ✅         | struct User { name: String }                 |
|                                      |                | let u = User {                               |
|                                      |                |     name: "Bob".to_string()                  |
|                                      |                | };                                           |
+--------------------------------------+----------------+----------------------------------------------+
| Need ownership / store value         | YES ✅         | let s: String = "hello".to_string();         |
+--------------------------------------+----------------+----------------------------------------------+
| Need to modify string                | YES ✅         | let mut s = "hi".to_string();                |
|                                      |                | s.push_str("!");                             |
+--------------------------------------+----------------+----------------------------------------------+
| Function parameter is &str           | NO ❌          | fn greet(name: &str) {                       |
|                                      |                |     println!("Hi {name}");                   |
|                                      |                | }                                            |
|                                      |                | greet("Alice");                              |
+--------------------------------------+----------------+----------------------------------------------+
| Just reading / printing              | NO ❌          | println!("{}", "blue");                     |
+--------------------------------------+----------------+----------------------------------------------+
| Passing String to &str param         | NO ❌          | let s = String::from("Bob");                 |
|                                      |                | greet(&s);                                   |
+--------------------------------------+----------------+----------------------------------------------+
| Temporary string literal (if/else)   | NO ❌          | let x = if ok { "yes" } else { "no" };       |
+--------------------------------------+----------------+----------------------------------------------+


 */