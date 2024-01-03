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
                "mov {0}, rax",
                out(reg) output
            );
        }
        assert_eq!(output, 8);
    }
  
}