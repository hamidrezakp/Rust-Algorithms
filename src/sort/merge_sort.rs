use super::Ordering;

/// Sort an slice with `merge_sort` algorithm
pub fn merge_sort<T: Copy + Ord>(slice: &mut [T], order: Ordering) {
    // base case
    match slice.len() {
        0 | 1 => return,
        _ => {
            let s_len = slice.len();
            merge_sort(&mut slice[0..(s_len / 2)], order);
            merge_sort(&mut slice[(s_len / 2)..s_len], order);

            let mut result_vec: Vec<T> = slice.to_vec();

            merge(
                &slice[0..(s_len / 2)],
                &slice[(s_len / 2)..s_len],
                &mut result_vec,
                order,
            );

            slice.copy_from_slice(&result_vec);
        }
    }
}

/// Merge two slice and make an biger slice from it.
/// Asuming two slices are neighbers and are from a bigger slice.
/// also two slices are asumed to be sorted.
fn merge<T: Copy + Ord>(first: &[T], second: &[T], result: &mut [T], order: Ordering) {
    assert_eq!(first.len() + second.len(), result.len());

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < first.len() && j < second.len() {
        match order {
            Ordering::Accending => {
                if first[i] < second[j] {
                    result[k] = first[i];
                    i += 1;
                    k += 1;
                } else {
                    result[k] = second[j];
                    j += 1;
                    k += 1;
                }
            }
            Ordering::Deccening => {
                if first[i] > second[j] {
                    result[k] = first[i];
                    i += 1;
                    k += 1;
                } else {
                    result[k] = second[j];
                    j += 1;
                    k += 1;
                }
            }
        }
    }
    if i < first.len() {
        result[k..].copy_from_slice(&first[i..]);
    }
    if j < second.len() {
        result[k..].copy_from_slice(&second[j..]);
    }
}

#[test]
fn test_merge_sort_empty_slice() {
    let mut array = [0u8; 0];
    merge_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0u8; 0]);
}

#[test]
fn test_merge_sort_one_element() {
    let mut array = [0];
    merge_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0]);
}

#[test]
fn test_merge_sort_two_element() {
    let mut array = [2, 1];
    merge_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [1, 2]);
}

#[test]
fn test_merge_sort_multiple_element() {
    let mut array = [4, 24, 12, 0, 445, 2, 1];
    merge_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_merge_sort_multiple_element_sorted() {
    let mut array = [0, 1, 2, 4, 12, 24, 445];
    merge_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_merge_sort_two_element_des() {
    let mut array = [1, 2];
    merge_sort(&mut array, Ordering::Deccening);
    assert_eq!(array, [2, 1]);
}

#[test]
fn test_merge_sort_multiple_element_des() {
    let mut array = [4, 24, 12, 0, 445, 2, 1];
    merge_sort(&mut array, Ordering::Deccening);
    assert_eq!(array, [445, 24, 12, 4, 2, 1, 0]);
}

#[test]
fn test_merge_sort_multiple_element_sorted_des() {
    let mut array = [445, 24, 12, 4, 2, 1, 0];
    merge_sort(&mut array, Ordering::Deccening);
    assert_eq!(array, [445, 24, 12, 4, 2, 1, 0]);
}
