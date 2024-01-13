use std::usize;

fn find_duplicate(arr: &Vec<i32>) {
    let mut ptr1 = arr[0] as usize;
    let mut ptr2 = arr[0] as usize;

    loop {
        // ptr1 step 1 node at a time
        ptr1 = arr[ptr1] as usize;

        // ptr2 step 2 nodes at a time
        ptr2 = arr[ptr2] as usize;
        ptr2 = arr[ptr2] as usize;

        if ptr1 == ptr2 {
            break;
        }
    }

    ptr1 = arr[0].clone() as usize;
    while ptr1 != ptr2 {
        ptr1 = arr[ptr1] as usize;
        ptr2 = arr[ptr2] as usize;
    }

    println!("Array = {:?}", arr);
    println!("Duplicate number = {}", ptr1);
}

fn main() {
    let array = vec![3, 3, 3, 2, 1, 3];
    find_duplicate(&array);
}
