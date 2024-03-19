pub fn remove_whitespace(lines: &Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    for line in lines {
        ret.push(line.trim_start().trim_end().to_string());
    }
    ret
}

pub fn remove_comments(lines: &Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    for line in lines {
        if line.starts_with(";") {
            continue;
        }
        let mut new_line = String::new();
        for c in line.chars() {
            if c == ';' {
                break;
            }
            new_line.push(c);
        }
        ret.push(new_line);
    }
    ret
}

pub fn remove_empty_lines(lines: &Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    for line in lines {
        // has only whitespace
        if line.trim().is_empty() {
            continue;
        }
        ret.push(line.clone());
    }
    ret
}

