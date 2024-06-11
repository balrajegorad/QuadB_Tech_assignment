//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.




fn first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        
        if nums[mid] == target {
            result = Some(mid);
            high = mid - 1; 
        } else if nums[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let nums = vec![1, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 4;
    if let Some(index) = first_occurrence(&nums, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array", target);
    }
}
