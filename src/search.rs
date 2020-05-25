/// doing a bianry search of item over slice of type `T` where `T` implement `Eq`
/// and `PartialEq` trait.
/// returns Some indice of `item` in `slice` if found, else None
pub fn binary_search<T>(slice: &[T], item: &T) -> Result<Option<usize>, &'static str>
where
    T: PartialEq + PartialOrd,
{
    if !slice.is_sorted() {
        return Err("Slice must be sorted!");
    }

    let mut start = 0;
    let mut end = slice.len() - 1;

    let mut middle_indice;
    let mut middle_item;

    // while there is something in range of `start` and `end`
    while start <= end {
        middle_indice = (start + end) / 2;
        middle_item = &slice[middle_indice];

        if middle_item == item {
            return Ok(Some(middle_indice)); // we are done cowboy
        } else if middle_item > item {
            end = middle_indice - 1; // we gone so far
        } else {
            start = middle_indice + 1; // come'n, we are too behind
        }
    }

    Ok(None)
}

#[test]
fn test_binary_search_not_found() {
    let array = [1, 2, 3, 4, 5, 6];
    let item = 9;

    assert_eq!(Ok(None), binary_search(&array, &item));
}

#[test]
fn test_binary_search_not_sorted() {
    let array = [2, 5, 3, 4, 5, 6];
    let item = 6;

    assert_eq!(Err("Slice must be sorted!"), binary_search(&array, &item));
}

#[test]
fn test_binary_search_found() {
    let array = [1, 2, 3, 4, 5, 6];
    let item = 6;

    assert_eq!(Ok(Some(5)), binary_search(&array, &item));
}
