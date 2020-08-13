pub mod house;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::house::front_of_house;
    #[test]
    fn test_add_to_waitlist() {
        let is_added = front_of_house::add_to_waitlist();
        // assert_eq!(is_added, false);
        assert_eq!(is_added, true);
    }
}
