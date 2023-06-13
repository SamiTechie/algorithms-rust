pub fn binary_search(list: &[u32], target_value: &u32) -> Option<usize>{
    let mut low :usize= 0;
    let mut high :usize= list.len();
    while low != high{
        let mid: usize = (high + low) /2;
        if list[mid] == *target_value{
            return Some(mid);
        }
        if list[mid] < *target_value{
            low = mid + 1;
        }else{
            high = mid;
        }
    }
    None
}

