use std::cmp::Ordering;
use std::collections::HashMap;
pub fn char_frequency(input: &str) -> Vec<(char, u32)> {
    let input_string = input.chars();
    let mut freq_table: HashMap<char, u32> = HashMap::new();
    for i in input_string {
        if !freq_table.contains_key(&i) {
            freq_table.insert(i, 1);
        } else {
            let v = freq_table.get(&i).unwrap();
            freq_table.insert(i, v + 1);
        }
    }
    let mut res: Vec<(char, u32)> = freq_table.into_iter().collect();
    res.sort_by(compare);
    res
}
fn compare(a: &(char, u32), b: &(char, u32)) -> Ordering {
    if a.1 > b.1 {
        return Ordering::Less;
    } else if a.1 < b.1 {
        return Ordering::Greater;
    }

    if a.0 < b.0 {
        return Ordering::Less;
    } else if a.0 > b.0 {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mississippi() {
        assert_eq!(
            char_frequency("mississippi"),
            vec![('i', 4), ('s', 4), ('p', 2), ('m', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(char_frequency(""), vec![]);
    }

    #[test]
    fn all_tied() {
        assert_eq!(char_frequency("abcabc"), vec![('a', 2), ('b', 2), ('c', 2)]);
    }

    #[test]
    fn single_char() {
        assert_eq!(char_frequency("z"), vec![('z', 1)]);
    }

    #[test]
    fn unique_chars_sorted_alphabetically() {
        assert_eq!(char_frequency("cba"), vec![('a', 1), ('b', 1), ('c', 1)]);
    }

    #[test]
    fn one_dominant_char() {
        assert_eq!(char_frequency("aaaaab"), vec![('a', 5), ('b', 1)]);
    }

    #[test]
    fn spaces_count_too() {
        assert_eq!(char_frequency("a a"), vec![('a', 2), (' ', 1)]);
    }

    #[test]
    fn six_unique_letters_sorted() {
        assert_eq!(
            char_frequency("fedcba"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1), ('f', 1)]
        );
    }
}
