
pub mod examples;

pub fn shadow_correct(shadow: &str) {
    let orig_shadow = shadow;
    let shadow = &format!("I'm shadowed in this new function, not changed!");

    // Test the shadowed variable's value vs. the original value
    // this proves it is actually shadowed. However, the original
    // shadow variable still exists and has not been changed!
    assert_ne!(shadow, orig_shadow)
}

pub fn double_wrong(shadow: u8) {
    // Do the doubling by creating a shadow variable and doubling the
    // original variable's value. This is the wrong way to do it!
    let _shadow = 2 * shadow;
}

// This function will show how a workaround to the double_wrong function if you
// really must do it that way. However, this is NOT recommended due to the fact
// that it is more memory intensive and less efficient.
pub fn double_wrong_workaround(shadow: u8) -> u8{
    // Do the doubling by creating a shadow variable and doubling the
    // original variable's value.
    let shadow = 2 * shadow;

    // Return the doubled value
    shadow
}

// This function looks like it should fix the shadowing issue, but it doesn't
// because the shadow is still passed by value due to it being a copy type!
pub fn fix_double_wrong(mut _shadow: u8) {
    _shadow = 2 * _shadow;
}

// This function is how you properly fix the shadowing issue with a copy type
pub fn fix_double_correct(shadow: &mut u8) {
    *shadow *= 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_wrong() {
        // Create a variable to shadow
        let shadow = 2;

        // Shadows the variable in the function,
        // but doesn't change the original value as is intended.
        double_wrong(shadow);

        // Prove that the original value is unchanged, it it were the test
        // would fail!
        assert_ne!(shadow, 4);
    }

    // We've marked this test as a panic because it should fail!
    #[should_panic]
    #[test]
    fn test_fix_double_wrong() {
        // Create a variable to shadow
        let shadow = 2;

        // Shadows and doubles the variable in the function,
        // but does not change the original value as we would intend.
        fix_double_wrong(shadow);

        // Prove that the original value is not changed, if it were
        // the test would pass!
        assert_eq!(shadow, 4);
    }

    #[test]
    fn test_double_wrong_workaround() {
        // Create a variable to shadow
        let mut shadow = 4;

        // Set the shadow to the new value.
        shadow = double_wrong_workaround(shadow);

        // Prove that the original value is changed.
        assert_eq!(shadow, 8);
    }

    #[test]
    fn test_fix_double_correct() {
        // Create a variable to shadow
        let mut shadow = 2;

        // Does not shadow the variable, as that is the wrong way to
        // do this. Instead, we pass a mutable reference and change
        // it in the function.
        fix_double_correct(&mut shadow);

        // Prove that the original value is changed, if it were not
        // this would fail!
        assert_eq!(shadow, 4);
    }

    #[test]
    fn test_shadow_correct() {
        // Create a variable to shadow
        let shadow = "I'm shadow";

        // Shadow the variable in the function and do an internal
        // assert to show that it was shadowed correctly.
        shadow_correct(shadow);

        // Prove that the shadowed variable is not changed leaving the
        // original value intact.
        assert_eq!(shadow, "I'm shadow");
    }

    #[test]
    fn test_recommended() {
        let mut recommended: u64 = 10;
        
        let double = move |num: &mut u64| {
            *num *= 2;
        };

        double(&mut recommended);

        assert_eq!(recommended, 20);
    }
}
