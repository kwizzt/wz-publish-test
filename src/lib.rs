//! Test publish crate

/// Add one to the input
///
/// # Example
/// ```
/// use wz_publish_test::add_one;
/// let res = add_one(5);
/// assert_eq!(6, res)
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
