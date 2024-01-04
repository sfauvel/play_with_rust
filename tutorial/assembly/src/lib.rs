

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
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
        // No need to run with --nocapture or --show-output. The output is not captured by rust test framework.
        // Because tests are executing in parallel, the output is randomly display. 
        // We can add the option `--test-threads=1` to be sure that the output is after the test name.
        let _output: u64;
        io::stdout().flush().unwrap();
        unsafe {
            asm!(
                // Put 4 chars on the stack
                "xor rax, rax",
                "add rax, 'M'",
                "shl rax, 8",
                "add rax, 'L'",
                "shl rax, 8",
                "add rax, 'K'",
                "shl rax, 8",
                "add rax, 'J'",
                "push rax",

                // Put 8 chars in one address on the stack
                "xor rax, rax",         // Init to 0
                "add rax, 'H' << 8*3",  // Put 8 bits in fourth position of a 32 bits register
                "add rax, 'G' << 8*2",
                "add rax, 'F' << 8*1",
                "add rax, 'E' << 8*0",
                // Need to shift the first 32 bits because we could not shift a 64 bits value with <<
                "shl rax, 8*4",         

                "add rax, 'D' << 8*3",
                "add rax, 'C' << 8*2",
                "add rax, 'B' << 8*1",
                "add rax, 'A' << 8*0",
                "push rax",

                "mov rsi, rsp", // message to write using the stack address
                "mov rdx, 12", // message length do not include the last 0
                "mov rdi, 1", // file descriptor: 1 = STDOUT
                "mov rax, 1", // system call number (sys_write)
                "syscall",

                "add rsp, 8*2", // Clean 2 push
            );
        }
        io::stdout().flush().unwrap();
    }
}