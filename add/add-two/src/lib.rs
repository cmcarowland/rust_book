#[cfg(test)]
mod add_two_tests {
    use super::*;

    #[test]
    fn add_two_test() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}