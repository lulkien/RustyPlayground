fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let last = digits.last().unwrap();
    let len = digits.len();
    if *last != 9 {
        digits[len - 1] = last + 1;
        digits
    } else {
        digits.reverse();
        let mut carry = 1;
        for value in digits.iter_mut() {
            if *value + carry == 10 {
                *value = 0;
                carry = 1;
            } else {
                *value += carry;
                carry = 0;
            }
        }

        if carry == 1 {
            digits.push(1);
        }

        digits.reverse();
        digits
    }
}

fn main() {
    let vec = vec![9, 9];
    let vec2 = plus_one(vec);
    println!("{:?}", vec2);
}
