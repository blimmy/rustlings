use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {

    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?

    let tx1 = tx.clone();

    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}


fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}


/*
    Arc can shared but cant edit so we hv to use mutex too
    Mutex<T> เพื่อความปลอดภัย
    - lock() = ขอสิทธิ์เข้าไปแก้ state
    - Rust ป้องกัน data race แบบ compile-time + runtime


    Ownership ของ channel Sender ถูก move เข้า thread แรก
Thread ที่สอง ใช้ไม่ได้ ถ้าไม่ clone
    mpsc = multi-producer
    producer หลายตัวส่งเข้า channel เดียวได้
receiver ตัวเดียว .clone() ไม่ได้ copy ค่า
copy สิทธิ์ในการส่ง channel ยังเป็นอันเดียวกัน


 */
