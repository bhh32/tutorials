pub mod variables;

#[cfg(test)]
mod tests {
    use variables::{collections::{vector_capacity_spec, vector_capcity_unspec}, floating_points::FloatingPoints, integers::Integers, strings::Strings};

    use super::*;

    #[test]
    fn test_i8() {
        let (min, max) = Integers::new().get_i8();

        assert_eq!(
            min,
            i8::MIN,
            "i8::MIN was not returned. Actual value was: {min}"
        );
        assert_eq!(
            max,
            i8::MAX,
            "i8::MAX was not returned. Actual value was: {max}"
        );
    }

    #[test]
    fn test_i16() {
        let (min, max) = Integers::new().get_i16();

        assert_eq!(
            min,
            i16::MIN,
            "i8::MIN was not returned. Actual value was: {min}"
        );
        assert_eq!(
            max,
            i16::MAX,
            "i8::MAX was not returned. Actual value was: {max}"
        );
    }

    #[test]
    fn test_i32() {
        let (min, max) = Integers::new().get_i32();

        assert_eq!(min, i32::MIN);
        assert_eq!(max, i32::MAX);
    }

    #[test]
    fn test_i64() {
        let (min, max) = Integers::new().get_i64();

        assert_eq!(min, i64::MIN);
        assert_eq!(max, i64::MAX);
    }

    #[test]
    fn test_i128() {
        let (min, max) = Integers::new().get_i128();

        assert_eq!(min, i128::MIN);
        assert_eq!(max, i128::MAX)
    }

    #[test]
    fn test_f32() {
        let (min, max) = FloatingPoints::new().get_f32();

        assert_eq!(min, f32::MIN);
        assert_eq!(max, f32::MAX);
    }

    #[test]
    fn test_f64() {
        let (min, max) = FloatingPoints::new().get_f64();

        assert_eq!(min, f64::MIN);
        assert_eq!(max, f64::MAX);
    }

    #[test]
    fn test_strings_struct() {
        let strings = Strings::new();

        assert!(!strings.get_string().is_empty());
        assert!(!strings.get_str_slice().is_empty());
    }

    #[test]
    fn test_get_slice_of_str() {
        let strings = Strings::new();

        let str_slice = strings.get_slice_of_str(0, 23);

        assert_eq!(str_slice, "This is a string slice!", "Actual Value: {str_slice}.");
    }

    #[test]
    fn test_vectors_with_capcity_unspec() {
        let vec = vector_capcity_unspec(12, "12th Man!");

        // Ensure that the size of the vector is the size that we gave.
        assert_eq!(vec.len(), 12, "Actual vec len: {}\nVec: {vec:#?}", vec.len());
        assert!(vec.capacity() >= 12, "Actual vec capacity: {}", vec.capacity());
    }

    #[test]
    fn test_vectors_with_capcity_spec() {
        let vec = vector_capacity_spec(42, "The answer!");

        assert_eq!(vec.capacity(), 42, "Actual vec capacity: {}", vec.capacity());

        vec.iter().for_each(|ele| {
            // Ensure all elements have the default value given.
            // The value is behind a double reference (&&str) so, we 
            // first must dereference it when checking.
            assert_eq!(*ele, "The answer!");
        })
    }
}
