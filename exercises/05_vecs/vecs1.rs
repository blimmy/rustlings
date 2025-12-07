fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    let v = vec![10, 20, 30, 40];


    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}


/*

    ** vec vs array

+------------------+---------------------+-------------------+
| Feature          | Vec<T>              | [T; N] (Array)    |
+------------------+---------------------+-------------------+
| Size             | Growable            | Fixed at compile  |
| Memory location  | Heap                | Stack             |
| Length known at  | Runtime             | Compile time      |
| Push / Pop       | Yes                 | No                |
| Passed to fn as  | &Vec<T> or &[T]     | &[T]              |
+------------------+---------------------+-------------------+


    ** vec vs slice
    
let v = vec![1, 2, 3, 4];              --> Vec<T> → owns the data
let s = &v[1..3];   // slice: [2, 3]   --> &[T] → borrowed view of data


    ** How to bulid vector
01 let v = vec![1, 2, 3]; recommend !!!!!!

02 new vec and push each :
    let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);

03 new vec but tell rust the capacity  - ช่วยลดการ reallocate
                                       - ใช้ใน code performance-critical
    let mut v = Vec::with_capacity(3);
        v.push(1);
        v.push(2);
        v.push(3);

04 สร้าง vector ที่มีค่าเริ่มต้นซ้ำ
    let v = vec![0; 5];  // [0, 0, 0, 0, 0]

05 แปลงจาก iterator / range

    let v: Vec<i32> = (0..5).collect();
    let v = (0..5).collect::<Vec<i32>>();

06 change from array
    let a = [1, 2, 3];
    let v = a.to_vec();

    or let v = Vec::from([1, 2, 3]);




 */
