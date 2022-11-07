pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // A binary search algorithm to find an element (key) in an array and return its index.
    
    let mut low: usize = 0;
    let mut high: usize = array.len();

    while low < high {
        let mid = (high + low) / 2;
        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Less => low = mid + 1
        }
    }
    None
}
