// expand push r0, r1, r2 to push r0, push r1, push r2, also the pop
pub fn expand_push_pop(lines: &Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    for line in lines {
        if line.starts_with("push") {
            let mut regs = line.split_whitespace().collect::<Vec<&str>>();
            regs.remove(0);
            for reg in regs {
                ret.push(format!("push {}", reg.trim_end_matches(',')));
            }
        } else if line.starts_with("pop") {
            let mut regs = line.split_whitespace().collect::<Vec<&str>>();
            regs.remove(0);
            for reg in regs {
                ret.push(format!("pop {}", reg.trim_end_matches(',')));
            }
        } else {
            ret.push(line.clone());
        }
    }
    ret
}

// expand the ite conditional instruction
// ite eq
// mov r0, #1
// mov r1, #2
// expand to
// beq =__IF_THEN_0:
// b =__IF_ELSE_0:
// __IF_THEN_0:
// mov r0, #1
// b =__IF_END_0
// __IF_ELSE_0:
// mov r1, #2
// __IF_END_0:
// can also be it, itt, itee, with the number of t and e being the number of instructions
pub fn expand_ite(lines: &Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    let mut it = lines.iter().peekable();
    let mut ite_count = 0;
    loop {
        let line = it.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.starts_with("it") {
            let splitted = line.split_whitespace().collect::<Vec<&str>>();
            let cond = splitted[1];
            let ite = splitted[0];

            let then_label = format!("__IF_THEN_{}", ite_count);
            let else_label = format!("__IF_ELSE_{}", ite_count);
            let end_label = format!("__IF_END_{}", ite_count);
            let t_count = ite.chars().filter(|c| *c == 't').count();
            let e_count: usize = ite.chars().filter(|c| *c == 'e').count();
            ret.push(format!("b{} ={}", cond, then_label));
            ret.push(format!("b ={}", else_label));
            ret.push(format!("{}:", then_label));
            for _ in 0..t_count {
                let line = it.next().unwrap();
                ret.push(line.clone());
            }
            ret.push(format!("b ={}", end_label));
            ret.push(format!("{}:", else_label));
            for _ in 0..e_count {
                let line = it.next().unwrap();
                ret.push(line.clone());
            }
            ret.push(format!("{}:", end_label));
            ite_count += 1;
        } else {
            ret.push(line.clone());
        }
    }
    ret
}