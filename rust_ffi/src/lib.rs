unsafe extern "C" {
    pub fn sum(a: i32, b: i32) -> i32;
    pub fn diff(a: i32, b: i32) -> i32;
    pub fn prod(a: i32, b: i32) -> i32;
    pub fn quot(a: i32, b: i32) -> i32;
}

unsafe extern "C" {
    pub fn add(a: i32, b: i32) -> i32;
    pub fn sub(a: i32, b: i32) -> i32;
    pub fn mul(a: i32, b: i32) -> i32;
    pub fn div(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        unsafe {
            assert_eq!(sum(2, 2), 4);
        }
    }

    #[test]
    fn test_diff() {
        unsafe {
            assert_eq!(diff(3, 2), 1);
        }
    }

    #[test]
    fn test_prod() {
        unsafe {
            assert_eq!(prod(4, 4), 16);
        }
    }

    #[test]
    fn test_quot() {
        unsafe {
            assert_eq!(quot(1, 1), 1);
            assert_eq!(quot(1, 0), 1234567890);
        }
    }

    #[test]
    fn test_add() {
        unsafe {
            assert_eq!(add(4, 4), 8);
        }
    }

    #[test]
    fn test_sub() {
        unsafe {
            assert_eq!(sub(5, 10), -5);
        }
    }

    #[test]
    fn test_mul() {
        unsafe {
            assert_eq!(mul(7, 6), 42);
        }
    }

    #[test]
    fn test_div() {
        unsafe {
            assert_eq!(div(100, 10), 10);
            assert_eq!(div(5, 0), 1234567890);
        }
    }
}
