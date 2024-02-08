pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

fn main() {
    let mut arr: Vec<i32> = vec![3,2,2,3, 4];
    let k = remove_element(&mut arr, 3);
    println!("{arr:?} {k}");
}
