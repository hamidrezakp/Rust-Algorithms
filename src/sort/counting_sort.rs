fn count_keys_equal(slice: &[u32], range: usize) -> Vec<u32> {
    let mut equal = vec![0u32; range];
    for i in 1..=slice.len() {
        equal[slice[i - 1] as usize] += 1;
    }
    equal
}

fn count_keys_less(equal: &[u32]) -> Vec<u32> {
    let mut less = vec![0u32; equal.len()];
    less[0] = 0;
    for i in 1..equal.len() {
        less[i] = less[i - 1] + equal[i - 1];
    }
    less
}

pub fn counting_sort(slice: &[u32], range: usize) -> Vec<u32> {
    if slice.len() < 2 {
        return Vec::from(slice);
    }
    let mut result = vec![0u32; slice.len()];
    let equal = count_keys_equal(slice, range);
    let less = count_keys_less(&equal);
    let mut next: Vec<u32> = less.iter().map(|x| x + 1).collect();

    for i in 1..=slice.len() {
        let key = slice[i - 1] as usize;
        let index = next[key] as usize;
        result[index - 1] = slice[i - 1];
        next[key] += 1;
    }
    result
}

#[test]
fn test_counting_sort_empty_slice() {
    let array = [0u32; 0];
    let result = counting_sort(&array, 1);
    assert_eq!(result, [0u32; 0]);
}

#[test]
fn test_counting_sort_one_element() {
    let array = [0];
    let result = counting_sort(&array, 1);
    assert_eq!(result, [0]);
}

#[test]
fn test_counting_sort_two_element() {
    let array = [2, 1];
    let result = counting_sort(&array, 3);
    assert_eq!(result, [1, 2]);
}

#[test]
fn test_counting_sort_multiple_element() {
    let array = [4, 24, 12, 0, 445, 2, 1];
    let result = counting_sort(&array, 446);
    assert_eq!(result, [0, 1, 2, 4, 12, 24, 445]);
}

#[test]
fn test_counting_sort_multiple_element_sorted() {
    let array = [0, 1, 2, 4, 12, 24, 445];
    let result = counting_sort(&array, 446);
    assert_eq!(result, [0, 1, 2, 4, 12, 24, 445]);
}
