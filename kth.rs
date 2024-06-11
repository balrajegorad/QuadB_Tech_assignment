//Implement a function that returns the kth smallest element in a given array

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; 
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort(); 

    Some(sorted_arr[k - 1]) 
}

fn main() {
    let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let k = 3;
    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is {}", k, smallest),
        None => println!("Invalid input"),
    }
}
