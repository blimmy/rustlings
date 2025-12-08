fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        // word = optional_target {
        //     assert_eq!(word, target);
        // }

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.

        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}


/*
    if-let
        “ถ้า EXPRESSION มีรูปแบบตรงกับ PATTERN ให้แกะค่าออกมาใช้”
    its the same as

    match optional_target {
    Some(word) => assert_eq!(word, target),
    None => {}
}

    // this one dont care case None
    if let Some(word) = optional_target {
    assert_eq!(word, target);
}


    while-let วน loop ตราบใดที่ EXPRESSION ยัง match กับ PATTERN”
    from Option<Option<i8>> to Some(Some(x))
    เพราะ pop() คืน Option<T> + T ในที่นี้คือ Option<i8>


 */