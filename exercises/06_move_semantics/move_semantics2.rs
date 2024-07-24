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
    use std::vec;

    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let _vec0_1 = vec0.clone();

        let vec1 = fill_vec(vec0);

        assert_eq!(_vec0_1, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
