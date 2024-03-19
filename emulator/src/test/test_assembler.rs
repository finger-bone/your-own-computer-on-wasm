#[cfg(test)]
mod test_assembler {
    use crate::{assembler::assemble::*, emulator::{instr::instr_to_string, CoreSys}};

    #[test]
    fn test_assembler_simple() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        mov r0, #1
        mov r1, #1
        add r0, r0, r1
        subs r1, r1, r0
        hlt
        "));
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(0), 2);
        assert_eq!(sys.get_reg(1), (-1 as i64) as u64);
    }

    #[test]
    fn test_mem() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        mov r0, #1
        mov r1, #1
        push r0, r1
        pop r2, r3
        sub sp, sp, #8
        str r3, sp
        ldr r4, sp
        add sp, sp, #8
        mov r0, #0
        ldr r5, r0, r1, #8
        hlt
        "));
        while !sys.halted() {
            println!("{}", instr_to_string(sys.get_next_instr()));
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(2), 1);
        assert_eq!(sys.get_reg(3), 1);
        assert_eq!(sys.get_reg(4), 1);
        assert_eq!(sys.get_reg(5), 0b1110010100000000000000010000000000000000000000000000000000000001);
    }
    #[test]
    fn test_cond() {
        let mut sys = CoreSys::new();
        println!("{}", preprocess("
        mov r0, #1
        mov r1, #1
        cmp r0, r1
        beq =label
        mov r0, #2
        label:
        it eq
        mov r1, #1
        hlt
        ".to_string()).join("\n"));
        sys = sys.load_mem(assemble("
        mov r0, #1
        mov r1, #1
        cmp r0, r1
        beq =label
        mov r0, #2
        label:
        it eq
        mov r1, #1
        ite ne
        mov r2, #2
        mov r3, #3
        hlt
        "));
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(0), 1);
        assert_eq!(sys.get_reg(1), 1);
        assert_ne!(sys.get_reg(2), 2);
        assert_eq!(sys.get_reg(3), 3);
    }
    #[test]
    fn test_raw_data() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        b =main
        data:
        .word
        1
        .asciz
        ab
        main:
        mov r2, =data
        ldr r0, r2
        hlt
        "));
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(0), 1);
    }
    #[test]
    fn test_mul() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        mov r0, #4
        mov r1, #4
        mul r2, r0, r1
        hlt
        "));
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(2), 16);
    }
    #[test]
    fn test_div() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        mov r0, #4
        mov r1, #4
        div r2, r0, r1
        hlt
        "));
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(2), 1);
    }
    #[test]
    fn test_recursion() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        mov r1, #16
        mov r0, #0
        nop
        bl =f
        hlt
        f:
        push lr
        subs r1, r1, #1
        beq =end
        push r1
        bl =f
        pop r1
        add r0, r0, r1
        pop lr
        b lr
        end:
        mov r0, #0
        pop lr
        b lr
        "));
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(0), 120);
    }
    #[test]
    fn test_int() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(assemble("
        mvi r0
        hlt
        "));
        sys = sys.set_int_table(vec![0, 0]);
        let mut interrupted = false;
        while !sys.halted() {
            sys = sys.step();
            if !interrupted {
                interrupted = true;
                sys = sys.interrupt(1, 2);
            }
        }
        assert_eq!(sys.get_reg(0), 2);
    }
}