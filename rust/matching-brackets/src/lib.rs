use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    let mapping: HashMap<_, _> = vec![('}', '{'), (']', '['), (')', '(')]
        .into_iter()
        .collect();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            ']' | '}' | ')' => {
                if stack.pop() != Some(*mapping.get(&c).unwrap()) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.len() == 0
}
