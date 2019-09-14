pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut start: i64 = 0;
    let mut end: i64 = (array.len() - 1) as i64;
    let mut mid: usize;
    while start <= end {
        mid = ((end + start) / 2) as usize;
        if array[mid as usize] > key {
            end = mid as i64 - 1;
        } else if array[mid as usize] < key {
            start = mid as i64 + 1;
        } else {
            return Some(mid as usize);
        }
    }
    None
}
