// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // `Arc` isn't enough if you want a **mutable** shared state.
    // We need to wrap the value with a `Mutex`.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    //                    ^^^^^^^^^^^                          ^

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock before you update a shared value.
            status_shared.lock().unwrap().jobs_done += 1;
            //           ^^^^^^^^^^^^^^^^
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
    //                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}

/*
สรุปแบบง่ายมาก: โค้ดนี้สอนเรื่อง “การใช้หลาย thread โดยไม่ให้ข้อมูลพัง”

1) ข้อมูลเดียวกัน แต่มีหลาย thread แก้ → อันตราย
   - ถ้าหลาย thread เขียนตัวแปรเดียวกัน อาจได้ค่ามั่ว (data race)
   - Rust จะไม่ยอมให้เขียนแบบอันตรายตั้งแต่ตอน compile

2) Arc<T> = แชร์ของให้หลาย thread ใช้
   - Arc ทำให้หลาย thread ถือข้อมูลก้อนเดียวกันได้
   - แต่ Arc อย่างเดียว “แก้ค่าไม่ได้” (อ่านได้อย่างเดียว)

3) Mutex<T> = ขออนุญาตก่อนแก้ข้อมูล
   - Mutex ทำหน้าที่เหมือน “กุญแจ”
   - ใครจะอ่านหรือแก้ ต้อง lock() ก่อน
   - ตอนหนึ่งมีได้แค่ thread เดียวที่ถือกุญแจ

4) Arc<Mutex<T>> = สูตรมาตรฐาน
   - Arc → เอาไปใช้หลาย thread ได้
   - Mutex → แก้ค่าได้อย่างปลอดภัย
   - ใช้คู่กันทุกครั้ง เมื่อหลาย thread ต้องแก้ข้อมูลเดียวกัน

5) lock() ทำให้ thread อื่นต้องรอ
   - ถ้ามี thread หนึ่งกำลังใช้ข้อมูลอยู่
   - thread อื่นต้องรอจนกว่าจะปล่อย lock
   - พอหลุดจาก scope → ปลดล็อกให้เอง

6) join() = รอให้ทุก thread ทำงานเสร็จ
   - main จะไม่เดินต่อจนกว่าทุก thread จะเสร็จ
   - ถ้าไม่ join → ผลลัพธ์อาจยังไม่ครบ

7) แค่ “อ่านค่า” ก็ต้อง lock()
   - ข้อมูลอยู่หลัง Mutex
   - จะอ่านหรือเขียนก็ต้องขอ lock() เสมอ

สรุปสั้นที่สุด:
หลาย thread จะแก้ข้อมูลเดียวกัน → ต้องใช้ Arc<Mutex<T>> เท่านั้น
*/
