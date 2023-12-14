
// Because it s use only in tests
#[allow(dead_code)]
fn my_func() -> u32 {
    42
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test_ok() {
        assert_eq!(42, my_func());
    }

  
}