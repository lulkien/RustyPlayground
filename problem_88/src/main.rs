use std::usize;

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut new_vec: Vec<i32> = Vec::new();
    let mut ptr1: usize = 0;
    let mut ptr2: usize = 0;

    loop {
        if ptr1 < m as usize && ptr2 < n as usize {
            if nums1[ptr1] <= nums2[ptr2] {
                new_vec.push(nums1[ptr1]);
                ptr1 += 1;
                continue;
            } else {
                new_vec.push(nums2[ptr2]);
                ptr2 += 1;
                continue;
            }
        }

        if ptr1 < m as usize {
            for idx in ptr1..m as usize {
                new_vec.push(nums1[idx]);
            }
            break;
        }

        if ptr2 < n as usize {
            for idx in ptr2..n as usize {
                new_vec.push(nums2[idx]);
            }
            break;
        }
    }

    nums1.clear();
    for element in new_vec {
        nums1.push(element);
    }
}

fn main() {
    let mut vec1 = vec![1, 0];
    let mut vec2 = vec![];
    merge(&mut vec1, 1, &mut vec2, 0);

    println!("Vec 1: {:?}", vec1);
    println!("Vec 2: {:?}", vec2);
}
