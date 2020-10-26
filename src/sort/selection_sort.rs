use super::Ordering;

pub fn selection_sort<T>(slice: &mut [T], order: Ordering) -> Result<(), &'static str>
where
    T: Ord,
{
    match order {
        Ordering::Accending => {
            for i in 0..slice.len() {
                match find_minimum(slice.get(i..).unwrap()) {
                    Some(val) => slice.swap(i + val, i),
                    _ => (),
                }
            }
        }
        Ordering::Deccening => {
            for i in 0..slice.len() {
                match find_maximum(slice.get(i..).unwrap()) {
                    Some(val) => slice.swap(i + val, i),
                    _ => (),
                }
            }
        }
    }
    Ok(())
}

fn find_minimum<T>(slice: &[T]) -> Option<usize>
where
    T: Ord,
{
    match slice.len() {
        0 => None,
        1 => Some(0),
        _ => {
            let mut minimum = 0;
            for i in 0..slice.len() {
                if slice[i] < slice[minimum] {
                    minimum = i;
                }
            }
            Some(minimum)
        }
    }
}

fn find_maximum<T>(slice: &[T]) -> Option<usize>
where
    T: Ord,
{
    match slice.len() {
        0 => None,
        1 => Some(0),
        _ => {
            let mut maximum = 0;
            for i in 0..slice.len() {
                if slice[i] > slice[maximum] {
                    maximum = i;
                }
            }
            Some(maximum)
        }
    }
}

#[test]
fn test_selection_sort_empty_slice() {
    let mut array = [0u8; 0];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Accending));
    assert_eq!(array, [0u8; 0]);
}

#[test]
fn test_selection_sort_one_element() {
    let mut array = [0];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Accending));
    assert_eq!(array, [0]);
}

#[test]
fn test_selection_sort_two_element() {
    let mut array = [2, 1];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Accending));
    assert_eq!(array, [1, 2]);
}

#[test]
fn test_selection_sort_multiple_element() {
    let mut array = [4, 24, 12, 0, 445, 2, 1];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Accending));
    assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_selection_sort_multiple_element_sorted() {
    let mut array = [0, 1, 2, 4, 12, 24, 445];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Accending));
    assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_selection_sort_two_element_des() {
    let mut array = [1, 2];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Deccening));
    assert_eq!(array, [2, 1]);
}

#[test]
fn test_selection_sort_multiple_element_des() {
    let mut array = [4, 24, 12, 0, 445, 2, 1];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Deccening));
    assert_eq!(array, [445, 24, 12, 4, 2, 1, 0]);
}

#[test]
fn test_selection_sort_multiple_element_sorted_des() {
    let mut array = [445, 24, 12, 4, 2, 1, 0];
    assert_eq!(Ok(()), selection_sort(&mut array, Ordering::Deccening));
    assert_eq!(array, [445, 24, 12, 4, 2, 1, 0]);
}
