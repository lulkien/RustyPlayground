use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // create a hash with key = element and value = index of it in vector
    let mut table: HashMap<i32, usize> = HashMap::new();

    for (index, value) in nums.into_iter().enumerate() {
        let diff = target - value;
        if let Some(&i) = table.get(&diff) {
            return vec![i as i32, index as i32];
        }
        table.insert(value, index);
    }
    vec![-1, -1]
}

fn main() {
    let arr = vec![2, 7, 11, 15];
    let target = 9;
    let two_sum = two_sum(arr, target);

    println!("{:?}", two_sum);
}
