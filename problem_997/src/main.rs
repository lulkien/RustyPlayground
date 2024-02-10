pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for num in nums {
        res.push(num * num);
    }
    res.sort_unstable();
    res
}

fn main() {
    let input = vec![-4, -2, -1, 0, 6, 99];
    let output = sorted_squares(input);

    println!("{:?}", output);
}
