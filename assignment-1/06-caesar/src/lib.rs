pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    let (input, shift) = (input, shift);
    let alpha_len = ALPHABET.len();
    if alpha_len == 0 {
        return input.to_string();
    }
    let shift = ((shift % alpha_len as i32) + alpha_len as i32) % alpha_len as i32;
    let shift_usize = shift as usize;
    let mut result = String::new();

    for c in input.chars() {
        match c {
            'a'..='z' => {
                let offset = (c as u32 - 'a' as u32) as usize;
                let new_idx = (offset + shift_usize) % alpha_len;

                result.push(ALPHABET.as_bytes()[new_idx] as char);
            }
            'A'..='Z' => {
                let offset = (c as u32 - 'A' as u32) as usize;
                let new_idx = (offset + shift_usize) % alpha_len;

                let new_char = ALPHABET.as_bytes()[new_idx] as char;
                result.push(new_char.to_ascii_uppercase());
            }
            _ => {
                result.push(c);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
