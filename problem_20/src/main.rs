use std::collections::HashMap;

fn is_valid(s: String) -> bool {
    let mut char_stack: Vec<char> = Vec::new();
    let mut char_table: HashMap<char, char> = HashMap::new();
    char_table.insert(')', '(');
    char_table.insert(']', '[');
    char_table.insert('}', '{');

    for c in s.chars().into_iter() {
        match c {
            '(' | '[' | '{' => {
                char_stack.push(c);
            }
            ')' | ']' | '}' => match char_table.get(&c) {
                Some(chr) => {
                    if Some(chr) == char_stack.last() {
                        char_stack.pop();
                    } else {
                        return false;
                    }
                }
                None => return false,
            },
            _ => {
                return false;
            }
        }
    }

    char_stack.is_empty()
}

fn main() {
    println!("{}", is_valid("[][]".to_string()));
}
