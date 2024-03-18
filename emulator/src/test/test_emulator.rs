use crate::emulator::CoreSys;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_plus_one() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r1, #1
            // mov r2, #1
            // add r0, r1, r2
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            0b1110_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            0b1110_0001, 0b0000_0000, 0b0001_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0010,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(1), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(2), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 2);
    }

    #[test]
    fn test_cond() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r0, #1
            // mov r1, #1
            // subs r0, r1, #1
            // moveq r2, #1
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            0b1110_1101, 0b0000_0000, 0b0010_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            0b0000_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(1), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 0);
        assert_eq!(sys.dump_cpsr(), 0b0100);
        sys = sys.step();
        assert_eq!(sys.get_reg(2), 1);
    }

    #[test]
    fn test_b() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // b #16
            // mov r1, #1
            // mov r2, #1
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            0b1110_0111, 0b0000_0000, 0b1000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0001_0000,
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            0b1110_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(15), 16);
        sys = sys.step();
        assert_ne!(sys.get_reg(1), 1);
        assert_eq!(sys.get_reg(2), 1);
    }

    #[test]
    fn test_fib() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r0, #0
            // mov r1, #1
            // mov r2, #0
            // mov r3, #0
            // LOOP:
            // add r2, r0, r1
            // mov r0, r1
            // mov r1, r2
            // add r3, r3, #1
            // cmp r3, #10
            // bne LOOP
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            
        ])

    }
}