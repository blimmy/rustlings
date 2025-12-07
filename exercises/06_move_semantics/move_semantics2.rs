fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}


/*

    and if we declare new vec ( like this one its
    vec1) and we said vec1 = vec0
    we cant call vec0 again !!!

    and how i use .clone

    from this, let vec1 = fill_vec(vec0);

    no: let vec1 = fill_vec(vec0).clone();
    yes: let vec1 = fill_vec(vec0.clone());

    put clone inside naja

 */