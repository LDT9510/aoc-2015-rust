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

pub struct IntParser<'a, T> {
    slice: &'a str,
    _phantom_data: PhantomData<T>,
}

pub struct NamedIntParser<'a, T> {
    slice: &'a str,
    _phantom_data: PhantomData<T>,
}

fn part_of_digit(byte: u8, curr_idx: usize, slice: &[u8]) -> bool {
    let is_negative_sign =
        curr_idx < slice.len() && (byte == b'-' && slice[curr_idx + 1].is_ascii_digit());

    is_negative_sign || byte.is_ascii_digit()
}

fn extract_int_indexes(bytes: &[u8]) -> (usize, usize) {
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
    (start_idx, end_idx)
}

pub trait IterInts {
    fn iter_ints<T: FromStr>(&self) -> IntParser<'_, T>;

    fn iter_named_ints<T: FromStr>(&self) -> NamedIntParser<'_, T>
    where
        <T as FromStr>::Err: std::fmt::Debug;
}

impl IterInts for &str {
    fn iter_ints<T: FromStr>(&self) -> IntParser<'_, T> {
        IntParser {
            slice: self,
            _phantom_data: PhantomData,
        }
    }

    fn iter_named_ints<T: FromStr>(&self) -> NamedIntParser<'_, T> {
        NamedIntParser {
            slice: self,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: FromStr> Iterator for IntParser<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.slice.as_bytes();
        let (start_idx, end_idx) = extract_int_indexes(bytes);
        let parsing_result = self.slice[start_idx..end_idx].parse::<T>().ok();
        self.slice = &self.slice[end_idx..];
        parsing_result
    }
}

impl<'a, T: FromStr> Iterator for NamedIntParser<'a, T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    type Item = (&'a str, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bytes = self.slice.as_bytes();

            let (start_idx, end_idx) = extract_int_indexes(bytes);

            if start_idx == 0 && end_idx == bytes.len() {
                break;
            }

            if start_idx <= 1 {
                self.slice = &self.slice[end_idx..];
                continue;
            }

            let integer = self.slice[start_idx..end_idx].parse::<T>().unwrap();
            let sub_str = self.slice[..start_idx - 1].split_ascii_whitespace();
            if let Some(last_word) = sub_str.last() {
                let name = if !last_word.chars().last().unwrap().is_alphabetic() {
                    &last_word[..last_word.len() - 1]
                } else {
                    last_word
                };

                if name.is_empty() {
                    self.slice = &self.slice[end_idx..];
                    continue;
                }

                self.slice = &self.slice[end_idx..];
                return Some((name, integer));
            }

            self.slice = &self.slice[end_idx..];
            continue;
        }

        None
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

    #[test]
    fn test_iter_named_ints() {
        let text = "1 hello: 99, rust 5 world: 445 7 thing? 4 some-text: 66666";
        let mut it = text.iter_named_ints();
        assert_eq!(Some(("hello", 99)), it.next());
        assert_eq!(Some(("rust", 5)), it.next());
        assert_eq!(Some(("world", 445)), it.next());
        assert_eq!(Some(("thing", 4)), it.next());
        assert_eq!(Some(("some-text", 66666)), it.next());
        assert_eq!(None, it.next());

        let text = "2";
        let mut it: NamedIntParser<i64> = text.iter_named_ints();
        assert_eq!(None, it.next());

        let text = "a 3434";
        let mut it: NamedIntParser<i64> = text.iter_named_ints();
        assert_eq!(Some(("a", 3434)), it.next());
        assert_eq!(None, it.next());

        let text = "- neg: -2434 - 5 -8";
        let mut it: NamedIntParser<i64> = text.iter_named_ints();
        assert_eq!(Some(("neg", -2434)), it.next());
        assert_eq!(None, it.next());
    }
}
