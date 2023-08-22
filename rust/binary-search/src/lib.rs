pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] > key {
            if mid > 0 {
                right = mid - 1;
            } else {
                return None;
            }
        } else {
            if left < array.len() - 1 {
                left = mid + 1;
            } else {
                return None;
            }
        }
    }
    None
}
