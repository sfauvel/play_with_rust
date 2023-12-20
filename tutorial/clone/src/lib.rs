
// Because it s use only in tests
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
fn pass_value_with_clone(mut value: WithCloneCopy) -> u32 {
    value.value += 1;
    value.value
}

#[allow(dead_code)]
struct WithoutClone {
    value: u32
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct WithCloneCopy {
    value: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behavior_use_value_without_clone() {
        let without_clone = WithoutClone{ value:12};
        assert_eq!(12, without_clone.value);

        let move_object = without_clone;
        assert_eq!(12, move_object.value);
        //assert_eq!(12, without_clone.value);  // error[E0382]: borrow of moved value: `without_clone`
    }

    #[test]
    fn behavior_use_value_with_clone_copy() {
        let mut with_clone: WithCloneCopy = WithCloneCopy{ value:12};
        assert_eq!(12, with_clone.value);

        // With Copy (and Clone), the value is not moved but copied.
        // Object are two distinct instances.
        let mut move_object = with_clone; 
        assert_eq!(12, move_object.value);
        assert_eq!(12, with_clone.value); 

        move_object.value = 15;
        assert_eq!(15, move_object.value);
        assert_eq!(12, with_clone.value); 

        with_clone.value = 17;
        assert_eq!(15, move_object.value);
        assert_eq!(17, with_clone.value); 
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
        let with_clone = WithCloneCopy{ value:12};
        assert_eq!(13, pass_value_with_clone(with_clone.clone()));
        assert_eq!(12, with_clone.value); 
    }

    #[test]
    fn behavior_implicit_cloning_value() {
        let with_clone = WithCloneCopy{ value:12};
        assert_eq!(13, pass_value_with_clone(with_clone));  // Make an implicit Copy (need Copy and Clone)
        assert_eq!(12, with_clone.value); 
    }
}