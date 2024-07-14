use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

pub fn buf_read() -> io::Result<HashMap<String, usize>> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut stack: Vec<String> = Vec::new();
    let mut occurrences: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('#') {
            continue;
        }

        if line.trim().is_empty() {
            // End of stack, so emit stack entry
            if !stack.is_empty() {
                let stack_key = stack.join(";");
                *occurrences.entry(stack_key).or_insert(0) += 1;
            }
            stack.clear();
        } else {
            stack.push(line);
        }
    }

    // Handle the last stack if the file doesn't end with an empty line
    if !stack.is_empty() {
        let stack_key = stack.join(";");
        *occurrences.entry(stack_key).or_insert(0) += 1;
    }

    Ok(occurrences)
}
