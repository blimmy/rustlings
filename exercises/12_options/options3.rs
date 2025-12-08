#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}


/*
    ใช้ Some(ref p) เมื่อค่าข้างในไม่ Copy และฉันยังต้องใช้ตัวแปรเดิมต่อ
    - Q1: ค่าที่อยู่ข้างใน Some(...) เป็น Copy ไหม?
    - Q2: หลัง match ยังต้องใช้ตัวแปรเดิมไหม?
 */