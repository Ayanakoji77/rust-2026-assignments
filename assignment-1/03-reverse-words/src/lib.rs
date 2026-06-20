pub fn reverse_words(sentence: &str) -> String {
    let mut input = sentence.char_indices().rev();
    let mut result = String::new();
    let mut flag = false;
    let mut right_index = 0;

    while let Some((index, c)) = input.next() {
        match c {
            ' ' | '\t' | '\n' => {
                if flag {
                    let word = &sentence[(index + 1)..right_index];

                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(word);
                    flag = false;
                }
            }

            _ => {
                if !flag {
                    right_index = index + 1;
                    flag = true;
                }
            }
        }
    }
    if flag {
        let word = &sentence[0..right_index];
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_words() {
        assert_eq!(reverse_words("hello world rust"), "rust world hello");
    }

    #[test]
    fn collapses_inner_whitespace() {
        assert_eq!(reverse_words("   one   two  "), "two one");
    }

    #[test]
    fn empty_input() {
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(reverse_words("single"), "single");
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(reverse_words("    "), "");
    }

    #[test]
    fn many_short_words() {
        assert_eq!(reverse_words("a b c d e"), "e d c b a");
    }

    #[test]
    fn tabs_and_newlines_count_as_whitespace() {
        assert_eq!(reverse_words("a\tb\nc"), "c b a");
    }

    #[test]
    fn leading_and_trailing_trim() {
        assert_eq!(
            reverse_words("  leading and trailing  "),
            "trailing and leading"
        );
    }
}
