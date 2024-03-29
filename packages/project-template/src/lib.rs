//! A project template for the quick instantiation of rust projects.

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
}
