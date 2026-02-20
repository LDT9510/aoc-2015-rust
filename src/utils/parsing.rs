use std::marker::PhantomData;
use std::str::FromStr;

pub trait UnwrapNextInt {
    fn unwrap_next_int<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug;
}

impl<'a, U: Iterator<Item = &'a str>> UnwrapNextInt for U {
    fn unwrap_next_int<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.next().unwrap().parse::<T>().unwrap()
    }
}

pub trait IterInts {
    fn iter_ints<T: FromStr>(&self) -> IntParser<'_, T>;
}

impl IterInts for &str {
    fn iter_ints<T: FromStr>(&self) -> IntParser<'_, T> {
        IntParser {
            slice: self,
            _phantom_data: PhantomData,
        }
    }
}

pub struct IntParser<'a, T> {
    slice: &'a str,
    _phantom_data: PhantomData<T>,
}

impl<T: FromStr> Iterator for IntParser<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        fn part_of_digit(byte: u8, curr_idx: usize, slice: &[u8]) -> bool {
            let is_negative_sign =
                curr_idx < slice.len() && (byte == b'-' && slice[curr_idx + 1].is_ascii_digit());

            is_negative_sign || byte.is_ascii_digit()
        }

        let bytes = self.slice.as_bytes();
        let mut start_idx = 0;
        let mut end_idx = bytes.len();
        let mut last_char_was_digit = false;

        for i in 0..bytes.len() {
            let b = bytes[i];
            let is_part_of_digit = part_of_digit(b, i, bytes);

            if is_part_of_digit && !last_char_was_digit {
                start_idx = i;
                last_char_was_digit = true;
            } else if !is_part_of_digit && last_char_was_digit {
                end_idx = i;
                break;
            }
        }

        let parsing_result = self.slice[start_idx..end_idx].parse::<T>().ok();
        self.slice = &self.slice[end_idx..];
        parsing_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_next_int() {
        let text = "a -1111 b 2222 3";
        let mut split = text.split_ascii_whitespace();
        // skip 'a'
        split.next();
        assert_eq!(split.unwrap_next_int::<i64>(), -1111);
        // skip 'b'
        split.next();
        assert_eq!(split.unwrap_next_int::<u64>(), 2222);
        assert_eq!(split.unwrap_next_int::<usize>(), 3);
    }

    #[test]
    fn test_iter_ints() {
        let mut v: Vec<i64> = "aaaa 123 bbbb3434 cccc -1123".iter_ints().collect();
        assert_eq!(v, vec![123, 3434, -1123]);

        v = "43432234".iter_ints().collect();
        assert_eq!(v, vec![43432234]);

        v = "aaaa 123 b-333 8".iter_ints().collect();
        assert_eq!(v, vec![123, -333, 8]);
    }
}
