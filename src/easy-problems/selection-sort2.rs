pub fn find_smallest<T: Ord>(arr: &[T]) -> usize {
    let mut smallest_index = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[smallest_index] {
            smallest_index = i;
        }
    }
    smallest_index
}

pub fn selection_sort<T: Ord + Clone>(arr: &mut Vec<T>) -> Vec<T> {
    let mut new_arr = Vec::with_capacity(arr.len());

    while !arr.is_empty() {
        let smallest_index = find_smallest(arr);
        // Remove the smallest element from `arr` and add it to `new_arr`
        new_arr.push(arr.remove(smallest_index));
    }

    new_arr
}

fn main() {
    let mut arr: Vec<u8> = vec![9, 5, 1, 3, 2, 7, 8];

    let result = selection_sort(&mut arr);

    println!("sorted array is: {:?}", result);
}
