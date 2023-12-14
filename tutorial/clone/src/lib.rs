
// Because it s use only in tests
#[allow(dead_code)]
fn my_func() -> u32 {
    42
}

// Because it s use only in tests
#[allow(dead_code)]
fn basic_value(value: u32) -> u32 {
    value
}

#[allow(dead_code)]
fn pass_value_without_clone(mut value: WithoutClone) -> u32 {
    value.value += 1;
    value.value
}

#[allow(dead_code)]
fn pass_ref_without_clone(value: &mut WithoutClone) -> u32 {
    value.value += 1;
    value.value
}


#[allow(dead_code)]
fn pass_value_with_clone(mut value: WithClone) -> u32 {
    value.value += 1;
    value.value
}

#[allow(dead_code)]
struct WithoutClone {
    value: u32
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct WithClone {
    value: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test_ok() {
        assert_eq!(42, my_func());
    }

    #[test]
    fn behavior_pass_value_without_clone() {
        let without_clone = WithoutClone{ value:12};
        assert_eq!(13, pass_value_without_clone(without_clone));
        // assert_eq!(13, withoutClone.value); // Not possible because we lose the ownership
    }

    #[test]
    fn behavior_pass_ref_without_clone() {
        let mut without_clone = WithoutClone{ value:12};
        assert_eq!(13, pass_ref_without_clone(&mut without_clone));
        assert_eq!(13, without_clone.value); 
    }

    #[test]
    fn behavior_cloning_value() {
        let with_clone = WithClone{ value:12};
        assert_eq!(13, pass_value_with_clone(with_clone.clone()));
        assert_eq!(12, with_clone.value); 
    }

    #[test]
    fn behavior_implicit_cloning_value() {
        let with_clone = WithClone{ value:12};
        assert_eq!(13, pass_value_with_clone(with_clone));  // Make an implicit Copy (need Copy and Clone)
        assert_eq!(12, with_clone.value); 
    }
}