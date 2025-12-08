use std::{collections::HashMap, ops::Index};

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &value) in nums.iter().enumerate() {
        let complement = target - value;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.entry(value).or_insert(i);
    }
    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 11;
    // match two_sum(&nums, target) {
    //     Some((i, j)) => println!("Indices: {} and {}", i, j),
    //     None => println!("No solution found"),
    // }

    match binary_search(&nums, target){
        Some(index) => println!("found {} at index {}", target, index),
        None => println!("{} not found", target),
    }
}


fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();


    while left < right {
        let mid = left + (right - left) / 2;
        let mid_value = arr[mid];

        if mid_value == target {
            return Some(mid);
        } else if target > mid_value {
            left = mid + 1;
        }else {
            right = mid;
        }
    }
    None
}
