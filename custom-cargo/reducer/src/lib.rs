#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, reduce(4));
    }
}

pub fn reduce(x: i32) -> i32 {
    x - 1
}
