fn my_sqrt(x: i32) -> i32 {
    return (x as f32).sqrt().floor() as i32;
}

fn main() {
    println!("{}", my_sqrt(10));
}
