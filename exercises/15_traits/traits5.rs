trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}

/*
compare_license_types(software1: impl Licensed, software2: impl Licensed)
this one dont have to + bc

software1 → ต้องเป็น Licensed

software2 → ต้องเป็น Licensed

แต่ไม่ได้บอกว่าต้องเป็น type เดียวกัน


(item: impl SomeTrait + OtherTrait)
= + แปลว่า “ต้องทำได้ทั้งหมด”, item ต้อง implement SomeTrait และ OtherTrait พร้อมกัน
 */
