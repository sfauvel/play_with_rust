


#[cfg(test)]
mod tests {
    use std::io::Read;
    use gag::BufferRedirect;
    use serial_test::serial;

    struct CustomSmartPointer {
        data: String,
    }
    
    impl CustomSmartPointer {
        fn build(value: &str) -> CustomSmartPointer {
            CustomSmartPointer {
                data: String::from(value),
            }
        }
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping `{}`", self.data);
        }
    }

    /// This test must be run with `--nocapture` otherwise println will no be intercept.
    /// Inner objet is dropped first.
    /// Next, objects are dropped inversely to their declarations.
    #[test]
    #[serial]
    fn xxx() {
        let mut buf = BufferRedirect::stdout().unwrap();
        {
            let a = CustomSmartPointer::build("first");
            let c = CustomSmartPointer {
                data: String::from("second"),
            };
            {
                let d = CustomSmartPointer {
                    data: String::from("inner"),
                };
            }
            let e = CustomSmartPointer::build("third");
            println!("CustomSmartPointers created.");
        }

        let mut line_from_stdout = String::new();
        buf.read_to_string(&mut line_from_stdout).unwrap();
        assert_eq!("Dropping `inner`\nCustomSmartPointers created.\nDropping `third`\nDropping `second`\nDropping `first`\n", line_from_stdout,
            "\n***WARNING***: This test must be run with the option --nocapture to pass.");

        drop(buf);
    }

    
 //  println!("Try to check this line");
 //  let mut line_from_stdout = String::new();
 //  buf.read_to_string(&mut line_from_stdout).unwrap();
 // 
 //  assert_eq!("Try to check this line\n", line_from_stdout);
//
 //  drop(buf);
 //  println!("stdout restored");
}