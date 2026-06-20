pub fn run_length_encode(input: &str) -> Vec<(char, u32)> {
    let mut fn_input = input.chars();
    let mut result: Vec<(char, u32)> = Vec::new();

    let mut prev = match fn_input.next() {
        Some(letter) => letter,
        None => return result,
    };
    let mut freq: u32 = 1;
    for c in fn_input {
        if c == prev {
            freq += 1;
        } else {
            result.push((prev, freq));
            freq = 1;
        }
        prev = c;
    }
    result.push((prev, freq));
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_aaabbc() {
        assert_eq!(
            run_length_encode("aaabbc"),
            vec![('a', 3), ('b', 2), ('c', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(run_length_encode(""), vec![]);
    }

    #[test]
    fn single_char() {
        assert_eq!(run_length_encode("x"), vec![('x', 1)]);
    }

    #[test]
    fn all_same() {
        assert_eq!(run_length_encode("aaaaa"), vec![('a', 5)]);
    }

    #[test]
    fn all_different() {
        assert_eq!(
            run_length_encode("abcd"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1)]
        );
    }

    #[test]
    fn alternating_runs() {
        assert_eq!(
            run_length_encode("aabbaa"),
            vec![('a', 2), ('b', 2), ('a', 2)]
        );
    }

    #[test]
    fn whitespace_counts_too() {
        assert_eq!(
            run_length_encode("aa  bb"),
            vec![('a', 2), (' ', 2), ('b', 2)]
        );
    }
}
