use std::collections::HashMap;

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
    let target = 9;
    match two_sum(&nums, target) {
        Some((i, j)) => println!("Indices: {} and {}", i, j),
        None => println!("No solution found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(&nums, 9), Some((0, 1)));
    }

    #[test]
    fn negative_numbers() {
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(&nums, 6), Some((1, 2)));
    }

    #[test]
    fn duplicates() {
        let nums = vec![3, 3];
        assert_eq!(two_sum(&nums, 6), Some((0, 1)));
    }
}
