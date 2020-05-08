#![feature(is_sorted)]

mod search {

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
}

pub mod sort {

    pub enum Ordering {
        Accending,
        Deccening,
    }

    pub fn selection_sort<T>(slice: &mut [T], order: Ordering) -> Result<(), &'static str>
    where
        T: Ord + std::fmt::Debug,
    {
        for i in 0..slice.len() {
            match find_smallest(slice.get(i..).unwrap()) {
                Some(val) => slice.swap(i + val, i),
                _ => (),
            }
        }
        Ok(())
    }

    fn find_smallest<T>(slice: &[T]) -> Option<usize>
    where
        T: Ord,
    {
        match slice.len() {
            0 => None,
            1 => Some(0),
            _ => {
                let mut smallest = 0;
                for i in 0..slice.len() {
                    if slice[i] < slice[smallest] {
                        smallest = i;
                    }
                }
                Some(smallest)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_not_found() {
        let array = [1, 2, 3, 4, 5, 6];
        let item = 9;

        assert_eq!(Ok(None), search::binary_search(&array, &item));
    }

    #[test]
    fn test_binary_search_not_sorted() {
        let array = [2, 5, 3, 4, 5, 6];
        let item = 6;

        assert_eq!(
            Err("Slice must be sorted!"),
            search::binary_search(&array, &item)
        );
    }

    #[test]
    fn test_binary_search_found() {
        let array = [1, 2, 3, 4, 5, 6];
        let item = 6;

        assert_eq!(Ok(Some(5)), search::binary_search(&array, &item));
    }

    #[test]
    fn test_selection_sort_empty_slice() {
        let mut array = [0u8; 0];
        assert_eq!(
            Ok(()),
            sort::selection_sort(&mut array, sort::Ordering::Accending)
        );
        assert_eq!(array, [0u8; 0]);
    }

    #[test]
    fn test_selection_sort_one_element() {
        let mut array = [0];
        assert_eq!(
            Ok(()),
            sort::selection_sort(&mut array, sort::Ordering::Accending)
        );
        assert_eq!(array, [0]);
    }

    #[test]
    fn test_selection_sort_two_element() {
        let mut array = [2, 1];
        assert_eq!(
            Ok(()),
            sort::selection_sort(&mut array, sort::Ordering::Accending)
        );
        assert_eq!(array, [1, 2]);
    }

    #[test]
    fn test_selection_sort_multiple_element() {
        let mut array = [4, 24, 12, 0, 445, 2, 1];
        assert_eq!(
            Ok(()),
            sort::selection_sort(&mut array, sort::Ordering::Accending)
        );
        assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
    }

    #[test]
    fn test_selection_sort_multiple_element_sorted() {
        let mut array = [0, 1, 2, 4, 12, 24, 445];
        assert_eq!(
            Ok(()),
            sort::selection_sort(&mut array, sort::Ordering::Accending)
        );
        assert_eq!(array, [0, 1, 2, 4, 12, 24, 445]);
    }
}
