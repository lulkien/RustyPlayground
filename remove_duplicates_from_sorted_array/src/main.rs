pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut cache: Option<i32> = None;
    for index in (0..nums.len()).rev() {
        if Some(nums[index]) == cache {
            nums.remove(index);
        } else {
            cache = Some(nums[index]);
        }
    }

    // Simple way =))
    // nums.dedup();
    nums.len() as i32
}

fn main() {
    let mut arr: Vec<i32> = vec![1, 2, 2, 3, 4, 4, 4, 4, 5];
    let k = remove_duplicates(&mut arr);
    println!("{:?}", arr);
    println!("{}", k);
}
