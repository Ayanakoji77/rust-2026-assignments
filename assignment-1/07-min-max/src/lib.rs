pub fn min_max(xs: &[i32]) -> Option<(i32, i32)> {
    let input = xs;
    if xs.is_empty() {
        return None;
    }
    let mut largest = input[0];
    let mut smallest = input[0];
    for &i in input {
        if largest < i {
            largest = i;
        }
        if smallest > i {
            smallest = i;
        }
    }
    Some((smallest, largest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_example() {
        assert_eq!(min_max(&[3, 1, 4, 1, 5, 9, 2, 6]), Some((1, 9)));
    }

    #[test]
    fn empty_slice() {
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn single_element() {
        assert_eq!(min_max(&[7]), Some((7, 7)));
    }

    #[test]
    fn all_equal() {
        assert_eq!(min_max(&[5, 5, 5, 5]), Some((5, 5)));
    }

    #[test]
    fn ascending() {
        assert_eq!(min_max(&[1, 2, 3, 4, 5]), Some((1, 5)));
    }

    #[test]
    fn descending() {
        assert_eq!(min_max(&[5, 4, 3, 2, 1]), Some((1, 5)));
    }

    #[test]
    fn negatives_only() {
        assert_eq!(min_max(&[-3, -1, -7, -2]), Some((-7, -1)));
    }

    #[test]
    fn i32_bounds() {
        assert_eq!(
            min_max(&[i32::MIN, 0, i32::MAX]),
            Some((i32::MIN, i32::MAX))
        );
    }
}
