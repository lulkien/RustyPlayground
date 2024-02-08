fn find_in_range(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> usize {
    if left >= right {
        if nums[left] >= target {
            return left;
        } else {
            return left + 1;
        }
    }

    let mid = (left + right) / 2;
    if nums[mid] > target {
        return find_in_range(nums, target, left, mid - 1);
    } else if nums[mid] < target {
        return find_in_range(nums, target, mid + 1, right);
    } else {
        return mid;
    }
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums[0] >= target {
        return 0;
    }

    if nums[&nums.len() - 1] == target {
        return nums.len() as i32 - 1;
    }

    if target > nums[nums.len() - 1] {
        return nums.len() as i32;
    }

    find_in_range(&nums, target, 0, nums.len()) as i32
}

fn main() {
    let arr: Vec<i32> = vec![1, 3, 5];
    let idx = search_insert(arr, 4);

    println!("{idx}");
}
