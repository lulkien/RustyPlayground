use std::cmp::max;

pub fn add_binary(a: String, b: String) -> String {
    let max_size = max(a.len(), b.len());
    let a_data: Vec<bool> = a
        .chars()
        .map(|c| c == '1')
        .rev()
        .chain(
            std::iter::repeat(false)
                .take(max_size - a.len())
                .collect::<Vec<_>>(),
        )
        .collect();

    let b_data: Vec<bool> = b
        .chars()
        .map(|c| c == '1')
        .rev()
        .chain(
            std::iter::repeat(false)
                .take(max_size - b.len())
                .collect::<Vec<_>>(),
        )
        .collect();

    let mut carry: bool = false;
    // let mut result: Vec<bool> = Vec::new();
    let mut result: String = String::with_capacity(max_size + 1);

    for index in 0..max_size {
        match (a_data[index], b_data[index], carry) {
            (true, true, true) => {
                result.insert(0, '1');
                carry = true;
            }
            (true, true, false) | (true, false, true) | (false, true, true) => {
                result.insert(0, '0');
                carry = true;
            }
            (false, false, true) | (false, true, false) | (true, false, false) => {
                result.insert(0, '1');
                carry = false;
            }
            (false, false, false) => {
                result.insert(0, '0');
                carry = false;
            }
        }
    }
    if carry == true {
        result.insert(0, '1');
    }

    result
}

fn main() {
    let binary = add_binary("111111".to_string(), "1".to_string());
    println!("{}", binary);
}
