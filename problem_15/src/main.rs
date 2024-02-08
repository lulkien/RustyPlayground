use std::collections::HashSet;

pub fn three_sum(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort();
    let mut existed_set: HashSet<Vec<i32>> = HashSet::new();

    let mut result: Vec<Vec<i32>> = Vec::new();
    for index in 0..arr.len() - 2 {
        let mut left = index + 1;
        let mut right = arr.len() - 1;
        while left < right {
            let sum = arr[index] + arr[left] + arr[right];
            if sum < 0 {
                left += 1;
            } else if sum > 0 {
                right -= 1;
            } else {
                let mut combo = vec![arr[index], arr[left], arr[right]];
                combo.sort();

                if existed_set.get(&combo) == None {
                    existed_set.insert(combo.clone());
                    result.push(combo);
                }

                left += 1;
                right -= 1;
            }
        }
    }

    result
}

fn main() {
    let arr = vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6];
    let result = three_sum(arr);
    println!("{:?}", result);
}
