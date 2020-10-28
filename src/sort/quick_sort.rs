use super::Ordering;
use std::fmt::Debug;

pub fn quick_sort<T: Copy + Ord + Debug>(slice: &mut [T], order: Ordering) {
    let s_len = slice.len();
    match s_len {
        // base case
        0 | 1 => return,
        _ => {
            eprintln!("Array before: {:?}", slice);
            let pivot_index = partition(slice, order);
            eprintln!("Pivot Index: {:?}", pivot_index);
            eprintln!("Array After: {:?}\n\n", slice);
            quick_sort(&mut slice[0..pivot_index], order);
            quick_sort(&mut slice[pivot_index + 1..s_len], order);
        }
    }
}

fn partition<T: Copy + Ord>(slice: &mut [T], order: Ordering) -> usize {
    let mut border = 0;
    let s_len = slice.len();

    for i in 0..(s_len - 1) {
        match order {
            Ordering::Accending => {
                if slice[i] <= slice[s_len - 1] {
                    slice.swap(border, i);
                    border += 1;
                }
            }
            Ordering::Deccening => {
                if slice[i] >= slice[s_len - 1] {
                    slice.swap(border, i);
                    border += 1;
                }
            }
        }
    }

    slice.swap(border, s_len - 1);
    // pivot index
    border
}

#[test]
fn test_quick_sort_empty_slice() {
    let mut array = [0u8; 0];
    quick_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0u8; 0]);
}

#[test]
fn test_quick_sort_one_element() {
    let mut array = [0];
    quick_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0]);
}

#[test]
fn test_quick_sort_two_element() {
    let mut array = [2, 1];
    quick_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [1, 2]);
}

#[test]
fn test_quick_sort_multiple_element() {
    let mut array = [4, 24, 12, 0, 445, 2, 1];
    quick_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_quick_sort_multiple_element_sorted() {
    let mut array = [0, 1, 2, 4, 12, 24, 445];
    quick_sort(&mut array, Ordering::Accending);
    assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_quick_sort_two_element_des() {
    let mut array = [1, 2];
    quick_sort(&mut array, Ordering::Deccening);
    assert_eq!(array, [2, 1]);
}

#[test]
fn test_quick_sort_multiple_element_des() {
    let mut array = [4, 24, 12, 0, 445, 2, 1];
    quick_sort(&mut array, Ordering::Deccening);
    assert_eq!(array, [445, 24, 12, 4, 2, 1, 0]);
}

#[test]
fn test_quick_sort_multiple_element_sorted_des() {
    let mut array = [445, 24, 12, 4, 2, 1, 0];
    quick_sort(&mut array, Ordering::Deccening);
    assert_eq!(array, [445, 24, 12, 4, 2, 1, 0]);
}
