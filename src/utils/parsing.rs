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
