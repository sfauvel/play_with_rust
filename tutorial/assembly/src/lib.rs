#[cfg(test)]
mod tests {
    use std::arch::asm;

    #[test]
    fn make_an_addition() {
        // From: https://doc.rust-lang.org/rust-by-example/unsafe/asm.html        
        let input: u64 = 3;
        let output: u64;
        unsafe {
            asm!(
                "mov {0}, {1}",
                "add {0}, 5",
                out(reg) output,
                in(reg) input,
            );
        }
        assert_eq!(output, 8);
    }

    #[test]
    fn make_an_addition_specifing_size() {
        let output: u64;
        unsafe {
            asm!(
                "mov {0}, {1:r}", // Because it don't know size of first parameter, we add `:r` (r for rax, e for eax)
                "add {0}, 5",
                out(reg) output,
                in(reg) 3,
            );
        }
        assert_eq!(output, 8);
    }

    #[test]
    fn make_an_addition_hard_coded() {
        let output: u64;
        unsafe {
            asm!(
                "mov rax, 3",
                "add rax, 5",
                out("rax") output
            );
        }
        assert_eq!(output, 8);
    }
  
    #[test]
    fn make_an_addition_modifing_variable() {
        // From: https://doc.rust-lang.org/rust-by-example/unsafe/asm.html        
        let input: u64 = 3;
        let output: u64;
        unsafe {
            asm!(
                "add {0}, 5",    // Use only one register
                inout(reg) input => output
            );
        }
        assert_eq!(output, 8);
    }



    extern "C" fn foo(arg: i32) -> i32 {
        println!("arg = {}", arg);
        arg + 3
    }
  
    #[test]
    fn call_foo() {
        let output: u64;
        unsafe {
            asm!(
                "mov rdi, 5",
                "call {}",
                // Function pointer to call
                in(reg) foo,             
                // Return value in rax
                out("rax") output,
                // Mark all registers which are not preserved by the "C" calling
                // convention as clobbered.
                clobber_abi("C"),
            );
        }

        assert_eq!(output, 8);
    }

    #[test]
    fn print_text() {
        let _output: u64;
        unsafe {
            asm!(
                "mov rax,4", // 'write' system call = 4",
                "mov rbx,1", // file descriptor 1 = STDOUT"
                "mov rdx,1", // length of string to write,
                "mov rcx, 80",
                "int 80h",
                out("rax") _,
               // out("rbx") _,
                out("rcx") _,
                out("rdx") _,
            );
        }
        //assert_eq!(output, 4);
    }
}