#[cfg(test)]
mod add_one_tests {
    use super::*;

    #[test]
    fn add_one_test() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}