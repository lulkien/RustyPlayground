fn is_palindrome(x: i32) -> bool {
    x.to_string().chars().rev().eq(x.to_string().chars())
}

fn main() {
    let number = 1234321;
    println!("{}", is_palindrome(number));
}
