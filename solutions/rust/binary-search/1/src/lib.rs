

fn binary_search<T, U>(low: usize, high: usize, array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: PartialOrd
{

    if low > high {
        None
    } else {
        let array = array.as_ref();
        let mid = (low+high)/2;
        if array[mid] == key {
            Some(mid)
        } else if array[mid] > key {
            binary_search(low, mid-1, array, key)
        } else {
            binary_search(mid+1, high, array, key)
        }
    }
}

pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
 U: PartialOrd {
     let array = array.as_ref();
    if array.is_empty() || array[0] > key {
        None
    } else {
        binary_search(0, array.len()-1, array, key)
    }
}
