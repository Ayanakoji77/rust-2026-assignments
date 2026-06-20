pub fn longest_word(sentence: &str) -> Option<&str> {
    let mut input = sentence.char_indices();

    let mut max_len = 0;
    let mut index_l = 0;
    let mut index_r = 0;

    let mut current_l = 0;
    let mut len;
    while let Some((index, c)) = input.next() {
        match c {
            ' ' => {
                len = index - current_l;
                if len > max_len {
                    max_len = len;
                    index_l = current_l;
                    index_r = index;
                }
                current_l = index + 1;
            }
            _ => {}
        }
    }
    let final_len = sentence.len() - current_l;
    if final_len > max_len && final_len > 0 {
        index_l = current_l;
        index_r = sentence.len();
        max_len = final_len;
    }
    if max_len == 0 {
        return None;
    }

    Some(&sentence[index_l..index_r])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest_of_four() {
        assert_eq!(longest_word("the quick brown fox"), Some("quick"));
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(longest_word("   "), None);
    }

    #[test]
    fn empty_input() {
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn ascending_lengths() {
        assert_eq!(longest_word("a bb ccc dd"), Some("ccc"));
    }

    #[test]
    fn single_word() {
        assert_eq!(longest_word("hello"), Some("hello"));
    }

    #[test]
    fn single_letter() {
        assert_eq!(longest_word("a"), Some("a"));
    }

    #[test]
    fn first_on_tie() {
        assert_eq!(longest_word("abc xyz def"), Some("abc"));
    }

    #[test]
    fn leading_and_trailing_whitespace() {
        assert_eq!(longest_word("  rust ferris  "), Some("ferris"));
    }
}
