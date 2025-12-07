// TODO: Change the line below to fix the compiler error.
const NUMBER:i32 = 3;

fn main() {
    println!("Number: {NUMBER}");
}


/*

+---------------------------+-------------------------------+---------------------------+
| Comparison Point          | const                         | let                       |
+---------------------------+-------------------------------+---------------------------+
| Can the value change?     | No                            | Yes (with `mut`)          |
| Type annotation required? | Yes                           | No (usually inferred)    |
| When is it evaluated?     | Compile time                  | Runtime                   |
| Can be global?            | Yes                           | No                        |
| Can use heap allocation?  | No                            | Yes                       |
| Must be a constant expr?  | Yes                           | No                        |
| Memory location           | Inlined / no real address     | Stack / Heap              |
+---------------------------+-------------------------------+---------------------------+


 */